use crate::config::AppConfig;
use anyhow::Context;

#[cfg(feature = "postgres")]
pub type DbPool = sqlx::PgPool;

#[cfg(feature = "sqlite")]
pub type DbPool = sqlx::SqlitePool;

#[cfg(feature = "postgres")]
static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations/postgres");

#[cfg(feature = "sqlite")]
static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations/sqlite");

pub async fn connect_and_migrate(cfg: &AppConfig) -> anyhow::Result<DbPool> {
  #[cfg(feature = "postgres")]
  {
    let pool = sqlx::postgres::PgPoolOptions::new()
      .max_connections(cfg.pool_max)
      .connect(&cfg.database_url)
      .await
      .with_context(|| "failed to connect to Postgres")?;
    MIGRATOR.run(&pool).await.with_context(|| "pg migrations failed")?;
    return Ok(pool);
  }

  #[cfg(feature = "sqlite")]
  {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
      .max_connections(cfg.pool_max)
      .connect(&cfg.database_url)
      .await
      .with_context(|| "failed to connect to SQLite")?;

    // recommended pragmas for write performance
    sqlx::query("PRAGMA journal_mode=WAL;").execute(&pool).await.ok();
    sqlx::query("PRAGMA synchronous=NORMAL;").execute(&pool).await.ok();

    MIGRATOR.run(&pool).await.with_context(|| "sqlite migrations failed")?;
    return Ok(pool);
  }
}
