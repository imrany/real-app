use std::error::Error;
use sqlx::Row;

#[tokio::main]
async fn main()->Result<(), Box<dyn Error>>{
    let url="postgres://admin:password123@0.0.0.0:6500/heallth";
    let pool=sqlx::postgres::PgPool::connect(url).await?;

    let res=sqlx::query("SELECT 1 + 1 as sum")
    .fetch_one(&pool)
    .await?;

    let sum:i32=res.get("sum");
    println!("1 + 1 ={}",sum);
    Ok(())
}