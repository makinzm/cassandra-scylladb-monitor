use clap::{Parser, Subcommand};
use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use std::env;

#[derive(Parser)]
#[command(about = "CQL client for Cassandra/ScyllaDB schema and data operations")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create keyspace and table if they do not exist
    DefineSchema,
    /// Insert or replace a user row (Cassandra INSERT is an upsert)
    Upsert {
        #[arg(long)]
        id: i32,
        #[arg(long)]
        name: String,
        #[arg(long)]
        email: String,
    },
    /// Print all rows in the users table
    Select,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let host = env::var("DB_HOST").unwrap_or_else(|_| "localhost:9042".to_string());
    let session: Session = SessionBuilder::new().known_node(&host).build().await?;

    match cli.command {
        Command::DefineSchema => define_schema(&session).await?,
        Command::Upsert { id, name, email } => upsert(&session, id, &name, &email).await?,
        Command::Select => select(&session).await?,
    }

    Ok(())
}

async fn define_schema(session: &Session) -> Result<(), Box<dyn std::error::Error>> {
    // SimpleStrategy with RF=1 is sufficient for a single-node dev/test cluster.
    session
        .query_unpaged(
            "CREATE KEYSPACE IF NOT EXISTS ks \
             WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1}",
            &[],
        )
        .await?;

    session
        .query_unpaged(
            "CREATE TABLE IF NOT EXISTS ks.users \
             (id int PRIMARY KEY, name text, email text)",
            &[],
        )
        .await?;

    println!("Schema ready: keyspace 'ks', table 'ks.users'");
    Ok(())
}

async fn upsert(
    session: &Session,
    id: i32,
    name: &str,
    email: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    session
        .query_unpaged(
            "INSERT INTO ks.users (id, name, email) VALUES (?, ?, ?)",
            (id, name, email),
        )
        .await?;

    println!("Upserted: id={id}  name={name}  email={email}");
    Ok(())
}

async fn select(session: &Session) -> Result<(), Box<dyn std::error::Error>> {
    let rows = session
        .query_unpaged("SELECT id, name, email FROM ks.users", &[])
        .await?
        .into_rows_result()?;

    for row in rows.rows::<(i32, String, String)>()? {
        let (id, name, email) = row?;
        println!("id={id}  name={name}  email={email}");
    }

    Ok(())
}
