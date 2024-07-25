use crate::mutation::Mutation;
use chrono::Utc;
use entity::reactions;
use sea_orm::sea_query::OnConflict;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait};

impl Mutation {
    pub async fn insert_reaction(db: &DbConn, cast: reactions::ActiveModel) -> anyhow::Result<()> {
        let _ = reactions::Entity::insert(cast)
            .on_conflict(OnConflict::new().do_nothing().to_owned())
            .exec(db)
            .await?;

        Ok(())
    }

    // insert multiple as once way raise "columns mismatch" error.
    pub async fn insert_reactions(
        db: &DbConn,
        casts: Vec<reactions::ActiveModel>,
    ) -> anyhow::Result<()> {
        let t = reactions::Entity::insert_many(casts)
            .on_conflict(OnConflict::new().do_nothing().to_owned())
            .exec(db)
            .await?;

        println!("insert_casts: {:?}", t);
        Ok(())
    }

    pub async fn delete_reaction(db: &DbConn, id: i32) -> anyhow::Result<()> {
        let mut cast: reactions::ActiveModel = reactions::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!("cast#{}", id)))
            .map(Into::into)?;

        cast.deleted_at = Set(Some(Utc::now().into()));
        cast.update(db).await?;

        Ok(())
    }
}
