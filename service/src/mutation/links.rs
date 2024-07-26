use crate::mutation::Mutation;
use chrono::Utc;
use entity::links;
use sea_orm::sea_query::OnConflict;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait};

impl Mutation {
    pub async fn insert_link(db: &DbConn, link: links::ActiveModel) -> anyhow::Result<()> {
        let _ = links::Entity::insert(link)
            .on_conflict(OnConflict::new().do_nothing().to_owned())
            .exec(db)
            .await?;

        Ok(())
    }

    pub async fn insert_links(db: &DbConn, casts: Vec<links::ActiveModel>) -> anyhow::Result<()> {
        let _ = links::Entity::insert_many(casts)
            .on_conflict(OnConflict::new().do_nothing().to_owned())
            .exec(db)
            .await?;

        Ok(())
    }

    pub async fn delete_link(db: &DbConn, id: i32) -> anyhow::Result<()> {
        let mut link: links::ActiveModel = links::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!("link#{}", id)))
            .map(Into::into)?;

        link.deleted_at = Set(Some(Utc::now().into()));
        link.update(db).await?;

        Ok(())
    }
}
