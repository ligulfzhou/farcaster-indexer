use crate::mutation::Mutation;
use entity::fids;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter};

impl Mutation {
    pub async fn insert_fid(db: &DbConn, fid: fids::ActiveModel) -> anyhow::Result<()> {
        fids::Entity::insert(fid)
            .on_conflict(
                OnConflict::column(fids::Column::Fid)
                    .update_columns(vec![
                        fids::Column::RegisterAt,
                        fids::Column::CustodyAddress,
                        fids::Column::RecoveryAddress,
                        fids::Column::UpdatedAt,
                    ])
                    .to_owned(),
            )
            .exec(db)
            .await?;

        Ok(())
    }

    pub async fn change_recovery(db: &DbConn, fid: fids::ActiveModel) -> anyhow::Result<()> {
        let mut f: fids::ActiveModel = fids::Entity::find()
            .filter(fids::Column::Fid.eq(fid.fid.into_value().unwrap()))
            .one(db)
            .await?
            .unwrap()
            .into();

        // Update name attribute
        f.custody_address = fid.custody_address;
        f.updated_at = fid.updated_at;

        f.update(db).await?;

        Ok(())
    }

    pub async fn transfer(db: &DbConn, fid: fids::ActiveModel) -> anyhow::Result<()> {
        let mut f: fids::ActiveModel = fids::Entity::find()
            .filter(fids::Column::Fid.eq(fid.fid.into_value().unwrap()))
            .one(db)
            .await?
            .unwrap()
            .into();

        // Update name attribute
        f.recovery_address = fid.recovery_address;
        f.updated_at = fid.updated_at;

        f.update(db).await?;

        Ok(())
    }
}