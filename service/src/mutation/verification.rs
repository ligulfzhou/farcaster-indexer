use crate::mutation::Mutation;
use chrono::Utc;
use entity::verifications;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sea_query::OnConflict;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter};

impl Mutation {
    pub async fn insert_verfications(
        db: &DbConn,
        data: Vec<verifications::ActiveModel>,
    ) -> anyhow::Result<()> {
        let res = verifications::Entity::insert_many(data)
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

    pub async fn delete_verfication_by_fid_signer(
        db: &DbConn,
        fid: i64,
        signer_address: &str,
        deleted_at: DateTimeWithTimeZone,
    ) -> anyhow::Result<()> {
        let mut f: verifications::ActiveModel = verifications::Entity::find()
            .filter(verifications::Column::Fid.eq(fid))
            .filter(verifications::Column::SignerAddress.eq(signer_address))
            .one(db)
            .await?
            .unwrap()
            .into();

        f.deleted_at = Set(Some(deleted_at));
        f.updated_at = Set(Utc::now().into());

        f.update(db).await?;

        Ok(())
    }
}
