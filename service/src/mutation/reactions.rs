use crate::mutation::Mutation;
use chrono::Utc;
use entity::reactions;
use sea_orm::sea_query::OnConflict;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter, Value};

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

    pub async fn delete_reaction(
        db: &DbConn,
        reaction: reactions::ActiveModel,
    ) -> anyhow::Result<()> {
        let target_url = reaction.target_url.into_value().unwrap();
        let target_cast_hash = reaction.target_cast_hash.into_value().unwrap();

        if let Value::String(Some(s)) = target_cast_hash {
            if (*s).eq("") {
                let mut res: reactions::ActiveModel = reactions::Entity::find()
                    .filter(reactions::Column::Fid.eq(reaction.fid.into_value().unwrap()))
                    .filter(reactions::Column::Type.eq(reaction.r#type.into_value().unwrap()))
                    .filter(reactions::Column::TargetUrl.eq(target_url.clone()))
                    .one(db)
                    .await?
                    .ok_or(DbErr::RecordNotFound(format!("reaction#{:?}", target_url)))
                    .map(Into::into)?;

                res.deleted_at = Set(Some(Utc::now().into()));
                res.update(db).await?;
            }
        } else {
            let mut res: reactions::ActiveModel = reactions::Entity::find()
                .filter(reactions::Column::Fid.eq(reaction.fid.into_value().unwrap()))
                .filter(reactions::Column::Type.eq(reaction.r#type.into_value().unwrap()))
                .filter(reactions::Column::TargetCastHash.eq(target_cast_hash.clone()))
                .one(db)
                .await?
                .ok_or(DbErr::RecordNotFound(format!(
                    "reaction#{:?}",
                    target_cast_hash
                )))
                .map(Into::into)?;

            res.deleted_at = Set(Some(Utc::now().into()));
            res.update(db).await?;
        }

        Ok(())
    }
}
