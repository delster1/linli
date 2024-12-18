use crate::database::{self, connect_to_db, db_create_user, db_add_post, user_add_server};
use crate::svrrequests::{AddPostRequest, AddUserRequest, GetPostsRequest, UserAddServerRequest};
use bytes::Bytes;
use chrono::Utc;
use http_body_util::Full;
use hyper::{body, Response, Request};
use std::collections::HashMap;
use std::net::IpAddr;
use rand::Rng;
use url::form_urlencoded;
pub struct Server {
    name: String,
    other_servers: Vec<String>, // Example information about the server
    db: sqlx::PgPool,
}
fn get_unix_time_as_bigint() -> i64 {
    Utc::now().timestamp_millis() // Get the current Unix time in milliseconds
}
impl Server {
    pub async fn new(name: &str, other_servers: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            other_servers: other_servers,
            db: connect_to_db().await.unwrap(),
        }
    }

    pub fn handle_std_request(&self) -> Result<Response<Full<Bytes>>, hyper::Error> {
        return Ok(hyper::Response::builder()
            .status(404)
            .body(Full::from(Bytes::from("Not Found")))
            .unwrap());
    }
    // i want handle_user to take []
    pub async fn handle_adduser(&self, req_bytes: Bytes) -> Result<Response<Full<Bytes>>, hyper::Error> {
        let parsed: AddUserRequest = match serde_json::from_slice(&req_bytes) {
            Ok(parsed) => parsed,
            Err(_) => {
                return Ok(Response::builder()
                    .status(400)
                    .body(Full::from(Bytes::from(
                        "Couldn't parse request & create user",
                    )))
                    .unwrap());
            }
        };

        println!(
            "username: {:?}. password: {:?}. salt: {:?}",
            parsed.username, parsed.password, parsed.salt
        );

        // ADD USER TO DATABASE
        db_create_user(
            &self.db,
            &parsed.username,
            &parsed.password,
            &parsed.salt,
            &self.name,
        ).await.unwrap();

        return Ok(Response::builder()
            .status(200)
            .body(Full::from(Bytes::from("OK")))
            .unwrap());
    }

    pub async fn handle_getposts(
        &self,
        req_bytes: Bytes,
    ) -> Result<Response<Full<Bytes>>, hyper::Error> 
    {

        let params: HashMap<String, String> =
            form_urlencoded::parse(&req_bytes).into_owned().collect();

        // Extract parameters
        let server = params.get("server").cloned().unwrap_or_default();
        let mut index = params
            .get("index")
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(0);

        let posts_result = database::db_get_posts(&self.db, server, index).await.unwrap(); // Fetch posts from the DB
        let posts = posts_result;
        let index_json = serde_json::to_string(&(index + 10)).unwrap();
        let posts_json = serde_json::to_string(&posts).expect("Failed to convert posts to JSON");
        let out_json = format!("{{\"index\":{},\"posts\":{}}}", index_json, posts_json);
        return Ok(Response::builder()
            .status(200)
            .body(Full::from(Bytes::from(out_json))) // Convert the JSON string to Bytes
            .unwrap());
    }

    pub async fn handle_addpost(&self, req_bytes: Bytes) -> Result<Response<Full<Bytes>>, hyper::Error> {
        let parsed: AddPostRequest = match serde_json::from_slice(&req_bytes) {
            Ok(parsed) => parsed,
            Err(_) => {
                return Ok(Response::builder()
                    .status(400)
                    .body(Full::from(Bytes::from("Could parse request & add post")))
                    .unwrap());
            }
        };
        let username = parsed.token;
        let post_content = parsed.text;
        println!("Adding post: {:?}", post_content);
        db_add_post(&self.db,  &username, &post_content).await.unwrap();
        return Ok(Response::builder()
            .status(200)
            .body(Full::from(Bytes::from("OK")))
            .unwrap());
    }
    pub async fn get_posts_from_server(
        ip: IpAddr,
        port: u16,
    ) -> Result<Response<Full<Bytes>>, hyper::Error> {
        // Use reqwest as the HTTP client
        let client = reqwest::Client::new();

        // Build the URL
        let url = format!("http://{}:{}/getposts", ip, port);

        // Send the request and fetch the response
        let res = client
            .get(&url)
            .send()
            .await
            .unwrap();

        // Ensure the response status is OK
        if !res.status().is_success() {
            eprintln!("Received error response: {}", res.status());
            return Ok(Response::builder()
                .status(500)
                .body(Full::from(Bytes::from("Failed to fetch posts from server")))
                .unwrap());
        }

        // Extract the response body as bytes
        let body = res
            .bytes()
            .await
            .unwrap();

        // Return the fetched posts
        Ok(Response::builder()
            .status(200)
            .body(Full::from(body))
            .unwrap())
    }

    pub async fn user_add_server(
        &self, 
        req_bytes: Bytes
    ) -> Result<Response<Full<Bytes>>, hyper::Error>
    {
        let parsed : UserAddServerRequest = match serde_json::from_slice(&req_bytes) {
            Ok(parsed) => parsed,
            Err(_) => {
                return Ok(Response::builder()
                    .status(400)
                    .body(Full::from(Bytes::from("Could parse request & add post")))
                    .unwrap());
            }
        };
        let username = parsed.username;
        let servername = parsed.servername;
        let password = parsed.password;

        user_add_server(&self.db, &username, &password, &servername).await.unwrap();
        return Ok(Response::builder()
            .status(200)
            .body(Full::from(Bytes::from("OK")))
            .unwrap());
        
    }

}

