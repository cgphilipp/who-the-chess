use sqlx::sqlite::SqliteConnectOptions;
use sqlx::ConnectOptions;

#[tokio::main]
async fn main() {
    let mut connection = SqliteConnectOptions::new()
        .create_if_missing(true)
        .filename("mydatabase.db")
        .connect()
        .await
        .expect("Could not open DB!");

    let _ = sqlx::query(
        "CREATE TABLE users (
        id INTEGER PRIMARY KEY,
        username TEXT NOT NULL,
        password TEXT NOT NULL
    )",
    )
    .execute(&mut connection)
    .await
    .expect("Could not create USERS");

    let _ = sqlx::query(
        "CREATE TABLE lobbies (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            owner_id INTEGER REFERENCES users (id),
            game_started BOOLEAN DEFAULT false
        )",
    )
    .execute(&mut connection)
    .await
    .expect("Could not create LOBBIES");
}
