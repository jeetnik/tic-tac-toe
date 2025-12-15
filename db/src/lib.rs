use std::env;
use anyhow::Result;
use sqlx::postgres::{PgPoolOptions, PgPool};
pub mod models;
struct Db{
    pool:PgPool

}

impl Db {
    pub  async  fn new()->Result<Self>{
        let db_url=env::var("DATABASE_URL")?;
        let pool=PgPoolOptions::new().max_connections(5).connect(&db_url).await?;
        Ok(Self{
            pool
        })

    }
    
}
