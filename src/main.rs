use sea_orm::{Database};
mod security;
mod entity;
mod errors;

#[tokio::main]
async fn main() {
    let connection = Database::connect("sqlite://main.sqlite?mode=rwc").await.unwrap(); 
} 
