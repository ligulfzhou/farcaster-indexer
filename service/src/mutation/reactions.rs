use crate::mutation::Mutation;
use chrono::Utc;
use entity::reactions;
use sea_orm::sea_query::OnConflict;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait};

impl Mutation {
    pub async fn insert_reaction(
        db: &DbConn,
        reaction: reactions::ActiveModel,
    ) -> anyhow::Result<()> {
        let res = reactions::Entity::insert(reaction)
            .on_conflict(
                OnConflict::column(reactions::Column::Hash)
                    .do_nothing()
                    .to_owned(),
            )
            .exec(db)
            .await;

        if let Err(err) = res {
            if err != DbErr::RecordNotInserted {
                return Err(anyhow::Error::new(err));
            }
        }

        Ok(())
    }

    pub async fn insert_reactions(
        db: &DbConn,
        reactions: Vec<reactions::ActiveModel>,
    ) -> anyhow::Result<()> {
        let res = reactions::Entity::insert_many(reactions)
            .on_conflict(
                OnConflict::column(reactions::Column::Hash)
                    .do_nothing()
                    .to_owned(),
            )
            .exec(db)
            .await;

        if let Err(err) = res {
            if err != DbErr::RecordNotInserted {
                return Err(anyhow::Error::new(err));
            }
        }

        Ok(())
    }

    pub async fn delete_reaction(db: &DbConn, id: i32) -> anyhow::Result<()> {
        let mut cast: reactions::ActiveModel = reactions::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!("reaction#{}", id)))
            .map(Into::into)?;

        cast.deleted_at = Set(Some(Utc::now().into()));
        cast.update(db).await?;

        Ok(())
    }
}
