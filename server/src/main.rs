use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::{Request, Response};
use hyper::server::conn::http1::Builder;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
mod database;
use hyper_util::rt::TokioIo;
use hyper::service::{service_fn, Service};
mod server;
mod svrrequests;
use crate::server::Server;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3333").await.unwrap();
    println!("Server listening on http://127.0.0.1:3333");

    // Wrap `Server` in an `Arc` for shared ownership
    let server = Arc::new(Server::new("server1", vec![]).await);
    loop {
        let (stream, _) = listener.accept().await?; // Use tokio::net::TcpStream
        let server = Arc::clone(&server);
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(err) = Builder::new()
                .serve_connection(
                    io,
                    service_fn(move |req| handle_request(req, Arc::clone(&server))),
                )
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn handle_request(
    req: Request<hyper::body::Incoming>,
    server: Arc<Server>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    match req.uri().path() {
        "/adduser" => {
            let whole_body = req.collect().await?.to_bytes();
            server.handle_adduser(whole_body).await
        }
        "/getposts" => {
            let whole_body = req.collect().await?.to_bytes();
            server.handle_getposts(whole_body).await
        }
        "/addpost" => {
            let whole_body = req.collect().await?.to_bytes();
            server.handle_addpost(whole_body).await
        }
        "/addserver" => {
            let whole_body = req.collect().await?.to_bytes();
            server.user_add_server(whole_body).await
        }
        _ => server.handle_std_request(),
    }
}
