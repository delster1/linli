use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AddUserRequest {
    pub username: String,
    pub password: String,
    pub salt: String,
}

#[derive(Deserialize)]
pub struct GetPostsRequest {
    pub server: String,
    pub index : i32,
}

#[derive(Deserialize)]
pub struct AddPostRequest {
    pub token: String,
    pub text: String,
}

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Post {
    pub author: String,
    pub text: String,
    pub likes: i32,
    pub date: i64,
    pub post_id: Option<i32>, 

}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
    port: u8
}
#[derive(Deserialize)]
pub struct UserAddServerRequest {
    pub username: String,
    pub password: String,
    pub servername: String,
 }
