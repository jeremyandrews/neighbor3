use anyhow::Result;
use dotenv::dotenv;
use sqlx::{Executor, PgPool};
use trillium::{Conn, State};
use trillium_logger::Logger;

#[tokio::main]
async fn main() -> Result<()> {
  // Load configuraiton.
  dotenv().ok();
  // Initialize logger.
  env_logger::init();

  // Connect to the database (as configured via DATABASE_URL).
  let db_pool = PgPool::connect(&dotenv::var("DATABASE_URL")?).await?;

  // Launch a Trillium server on port 8080.
  trillium_tokio::run_async((
    // Log connections.
    Logger::new(),
    // Pass the database to all threads.
    State::new(db_pool),
    // POC function.
    hello_world
  )).await;

  Ok(())
}

// POC function that returns "Hello world!".
async fn hello_world(conn: Conn) -> Conn {
  // Load the db_pool from the Trillium state.
  let db_pool = conn.state::<PgPool>().expect("failed to get db_pool");
  // Execute a database query.
  db_pool.execute(sqlx::query("SELECT * FROM person")).await.expect("failed to query person table");

  // Default output.
  conn.with_status(200)
    .with_header("x-neighbor", "3")
    .with_body("Hello world!")
}
