use entity::sea_orm::Database;
use service::sea_orm::{ConnectOptions, DbConn};

pub async fn get_db_conn(url: &str) -> anyhow::Result<DbConn> {
    let mut opt = ConnectOptions::new(url);
    opt.sqlx_logging(false);
    let db = Database::connect(opt)
        .await
        .expect("database connection failed.");

    Ok(db)
}
