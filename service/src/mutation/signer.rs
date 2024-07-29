use crate::mutation::Mutation;
use entity::signers;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter};
use std::vec;

impl Mutation {
    pub async fn insert_signer(db: &DbConn, signer: signers::ActiveModel) -> anyhow::Result<()> {
        let res = signers::Entity::insert(signer)
            .on_conflict(
                OnConflict::columns(vec![signers::Column::Fid, signers::Column::Key])
                    .update_columns(vec![
                        signers::Column::AddedAt,
                        signers::Column::RequesterFid,
                        signers::Column::KeyType,
                        signers::Column::Metadata,
                        signers::Column::MetadataType,
                        signers::Column::UpdatedAt,
                    ])
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

    pub async fn remove_signer(db: &DbConn, signer: signers::ActiveModel) -> anyhow::Result<()> {
        let mut f: signers::ActiveModel = signers::Entity::find()
            .filter(signers::Column::Fid.eq(signer.fid.into_value().unwrap()))
            .filter(signers::Column::Key.eq(signer.key.into_value().unwrap()))
            .one(db)
            .await?
            .unwrap()
            .into();

        f.removed_at = signer.removed_at;
        f.updated_at = signer.updated_at;

        f.update(db).await?;

        Ok(())
    }
}
