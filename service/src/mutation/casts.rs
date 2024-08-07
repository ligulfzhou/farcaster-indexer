use crate::mutation::Mutation;
use chrono::Utc;
use entity::casts;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sea_query::OnConflict;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter};

impl Mutation {
    pub async fn insert_cast(db: &DbConn, cast: casts::ActiveModel) -> anyhow::Result<()> {
        let res = casts::Entity::insert(cast)
            .on_conflict(
                OnConflict::column(casts::Column::Hash)
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

    // insert multiple as once way raise "columns mismatch" error.
    pub async fn insert_casts(db: &DbConn, casts: Vec<casts::ActiveModel>) -> anyhow::Result<()> {
        let res = casts::Entity::insert_many(casts)
            .on_conflict(
                OnConflict::column(casts::Column::Hash)
                    .do_nothing()
                    .to_owned(),
            )
            .exec(db)
            .await;

        if let Err(err) = res {
            if err != DbErr::RecordNotInserted {
                dbg!(&err);
                return Err(anyhow::Error::new(err));
            }
        }

        Ok(())
    }

    pub async fn delete_cast_by_hash(
        db: &DbConn,
        hash: &str,
        deleted_at: DateTimeWithTimeZone,
    ) -> anyhow::Result<()> {
        let mut cast: casts::ActiveModel = casts::Entity::find()
            .filter(casts::Column::Fid.eq(hash))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!("cast#{}", hash)))
            .map(Into::into)?;

        cast.deleted_at = Set(Some(deleted_at));
        cast.updated_at = Set(Utc::now().into());
        cast.update(db).await?;

        Ok(())
    }
}
