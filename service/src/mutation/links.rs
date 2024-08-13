use crate::mutation::Mutation;
use chrono::Utc;
use entity::{casts, links, reactions};
use sea_orm::sea_query::OnConflict;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter, QueryOrder};

impl Mutation {
    pub async fn insert_link(db: &DbConn, link: links::ActiveModel) -> anyhow::Result<()> {
        let res = links::Entity::insert(link)
            .on_conflict(OnConflict::new().do_nothing().to_owned())
            .exec(db)
            .await;

        if let Err(err) = res {
            if err != DbErr::RecordNotInserted {
                return Err(anyhow::Error::new(err));
            }
        }

        Ok(())
    }

    pub async fn insert_links(db: &DbConn, casts: Vec<links::ActiveModel>) -> anyhow::Result<()> {
        let res = links::Entity::insert_many(casts)
            .on_conflict(OnConflict::new().do_nothing().to_owned())
            .exec(db)
            .await;

        if let Err(err) = res {
            if err != DbErr::RecordNotInserted {
                return Err(anyhow::Error::new(err));
            }
        }

        Ok(())
    }

    pub async fn delete_link(db: &DbConn, link: links::ActiveModel) -> anyhow::Result<()> {
        let fid = link.fid.unwrap();
        let target_fid = link.target_fid.unwrap();
        let mut link_am: links::ActiveModel = links::Entity::find()
            .filter(links::Column::Fid.eq(fid))
            .filter(links::Column::TargetFid.eq(target_fid))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "link_with_fid_targetfid#{:?}, {:?}",
                fid, target_fid
            )))
            .map(Into::into)?;

        link_am.deleted_at = Set(Some(link.timestamp.unwrap()));
        link_am.updated_at = Set(Utc::now().into());
        link_am.update(db).await?;

        Ok(())
    }

    pub async fn prune_link(db: &DbConn, link: links::ActiveModel) -> anyhow::Result<()> {
        let fid = link.fid.unwrap();
        let target_fid = link.target_fid.unwrap();
        let mut link_am: links::ActiveModel = links::Entity::find()
            .filter(links::Column::Fid.eq(fid))
            .filter(links::Column::TargetFid.eq(target_fid))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "link_with_fid_targetfid#{:?}, {:?}",
                fid, target_fid
            )))
            .map(Into::into)?;

        link_am.pruned_at = Set(Some(link.timestamp.unwrap()));
        link_am.updated_at = Set(Utc::now().into());
        link_am.update(db).await?;

        Ok(())
    }
}
