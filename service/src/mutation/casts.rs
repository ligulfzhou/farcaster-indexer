use crate::mutation::Mutation;
use chrono::Utc;
use entity::casts;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait};

impl Mutation {
    pub async fn insert_cast(db: &DbConn) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn delete_cast(db: &DbConn, id: i32) -> anyhow::Result<()> {
        let mut cast: casts::ActiveModel = casts::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!("cast#{}", id)))
            .map(Into::into)?;

        cast.deleted_at = Set(Some(Utc::now().into()));

        cast.update(db).await?;

        Ok(())
    }
}