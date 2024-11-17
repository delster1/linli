use crate::svrrequests::Post;
use chrono::Utc;
use serde_json;
use sqlx::Row;
use sqlx::{postgres::PgRow, PgPool};
fn get_unix_time_as_bigint() -> i64 {
    Utc::now().timestamp_millis() // Get the current Unix time in milliseconds
}
// Connect to the database
pub async fn connect_to_db() -> Result<PgPool, sqlx::Error> {
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in the environment");
    PgPool::connect(&database_url).await
}

// Add a user to the database
pub async fn db_create_user(
    pool: &PgPool,
    username: &str,
    password: &str,
    salt: &str,
    servername: &str,
) -> Result<(), sqlx::Error> {
    // Serialize servername into a JSON array
    let servers = serde_json::json!([servername]);

    sqlx::query("INSERT INTO users (username, password, salt, servers) VALUES ($1, $2, $3, $4);")
        .bind(username)
        .bind(password)
        .bind(salt)
        .bind(servers) // Pass the JSON value directly
        .execute(pool)
        .await?;

    println!("User created: {}", username);
    Ok(())
}
// Get posts with pagination
pub async fn db_get_posts(
    pool: &PgPool,
    server: String,
    index: i32,
) -> Result<Vec<Post>, sqlx::Error> {
    let date_time = get_unix_time_as_bigint();
    // Fetch rows directly using LIMIT and OFFSET for pagination
    let rows = sqlx::query("SELECT id, text, likes, date, author FROM posts LIMIT 10 OFFSET $2;")
        .bind(server)
        .bind(index * 10) // Bind offset for pagination
        .fetch_all(pool) // Fetch all matching rows
        .await?;

    // Map the rows to the Post struct
    let posts: Vec<Post> = rows
        .into_iter()
        .map(|row| Post {
            post_id: row.try_get("id").unwrap_or_default(),
            text: row.try_get("text").unwrap_or_default(),
            likes: row.try_get("likes").unwrap_or_default(),
            date: row.try_get("date").unwrap_or_default(),
            author: row.try_get("author").unwrap_or_default(),
        })
        .collect();

    println!("Posts fetched: {:?}", posts);
    Ok(posts)
}

// Add a post to the database
pub async fn db_add_post(pool: &PgPool, token: &str, text: &str) -> Result<(), sqlx::Error> {
    let date_time = get_unix_time_as_bigint();
    sqlx::query("INSERT INTO posts (author, text, date) VALUES ($1, $2, $3);")
        .bind(token)
        .bind(text)
        .bind(date_time) // Use RFC3339 timestamp for clarity
        .execute(pool)
        .await?;

    println!("Post created: {}", text);
    Ok(())
}

pub async fn user_add_server(
    pool: &PgPool,
    username: &str,
    password: &str,
    servername: &str,
) -> Result<(), sqlx::Error> {
    // Step 1: Fetch the current list of servers
    let row: (Option<String>,) = sqlx::query_as(
        "SELECT servers FROM users WHERE username = $1 AND password = $2",
    )
    .bind(username)
    .bind(password)
    .fetch_one(pool)
    .await?;

    // Step 2: Parse the JSON list into a Vec<String>
    let mut servers: Vec<String> = match row.0 {
        Some(json) => serde_json::from_str(&json).unwrap_or_else(|_| vec![]),
        None => vec![],
    };

    // Step 3: Add the new server to the list
    if !servers.contains(&servername.to_string()) {
        servers.push(servername.to_string());
    }

    // Step 4: Serialize the updated list back into JSON
    let updated_servers = serde_json::to_string(&servers).unwrap();

    // Step 5: Update the database with the new list
    sqlx::query(
        "UPDATE users SET servers = $1 WHERE username = $2 AND password = $3",
    )
    .bind(updated_servers)
    .bind(username)
    .bind(password)
    .execute(pool)
    .await?;

    println!("Server added to user {}: {}", username, servername);
    Ok(())
}

