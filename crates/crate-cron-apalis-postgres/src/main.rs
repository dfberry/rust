use anyhow::Result;
use apalis::layers::retry::RetryPolicy;

use apalis::prelude::*;
use apalis_sql::postgres::{PgListen, PgPool, PostgresStorage};
use dotenvy::dotenv;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


// 3P service - somewhere else
#[derive(Clone)]
struct FakeService;
impl FakeService {
    fn execute(&self, item: Reminder) {
        println!("DOING Something: {:?}", item.scheduled_time);
    }
}

// Data struct from 3P service
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Reminder{
    pub scheduled_time: DateTime<Utc>
}
impl From<DateTime<Utc>> for Reminder {
    fn from(t: DateTime<Utc>) -> Self {
        Reminder { scheduled_time: t }
    }
}

// Apalis worker calls 3P service
async fn run_job(job: Reminder, svc: Data<FakeService>) {
    println!("Sending: {:?}", job.scheduled_time);
    svc.execute(job);
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug,sqlx::query=error");
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL").expect("Must specify path to db");

    if database_url.trim().is_empty() {
        panic!("DATABASE_URL environment variable is empty");
    }

    let pool = PgPool::connect(&database_url).await?;
    PostgresStorage::setup(&pool)
        .await
        .expect("unable to run migrations for postgres");

    let mut pg = PostgresStorage::new(pool.clone());
    let mut listener = PgListen::new(pool).await?;

    listener.subscribe_with(&mut pg);

    tokio::spawn(async move {
        listener.listen().await.unwrap();
    });

    Monitor::new()
        .register({
            WorkerBuilder::new("tasty-orange")
                .enable_tracing()
                .retry(RetryPolicy::retries(5))
                .backend(pg)
                .build_fn(run_job)
        })
        .on_event(|e| println!("{e}"))
        .run_with_signal(async {
            tokio::signal::ctrl_c().await?;
            println!("Shutting down the system");
            Ok(())
        })
        .await?;
    Ok(())
}