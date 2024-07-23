use crate::m20220101_000001_create_table::Fids::Fid;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Casts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Casts::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Casts::Fid).big_integer().not_null())
                    .col(ColumnDef::new(Casts::ParentFid).big_integer())
                    .col(ColumnDef::new(Casts::Hash).text().not_null().unique_key())
                    .col(ColumnDef::new(Casts::RootParentHash).text())
                    .col(ColumnDef::new(Casts::ParentHash).text())
                    .col(ColumnDef::new(Casts::RootParentUrl).text())
                    .col(ColumnDef::new(Casts::ParentUrl).text())
                    .col(ColumnDef::new(Casts::Text).text().not_null())
                    .col(
                        ColumnDef::new(Casts::Embeds)
                            .json()
                            .not_null()
                            .default("[]"),
                    )
                    .col(
                        ColumnDef::new(Casts::Mentions)
                            .json()
                            .not_null()
                            .default("[]"),
                    )
                    .col(
                        ColumnDef::new(Casts::MentionsPositions)
                            .json()
                            .not_null()
                            .default("[]"),
                    )
                    .col(
                        ColumnDef::new(Casts::CreateAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .col(
                        ColumnDef::new(Casts::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .col(
                        ColumnDef::new(Casts::Timestamp)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Casts::DeletedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Casts::PrunedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_casts_active_fid_timestamp")
                    .table(Casts::Table)
                    .col(Casts::Fid)
                    .col(Casts::Timestamp)
                    .col(Casts::DeletedAt) //     .where(sql.ref('deleted_at'), 'is', null) // Only index active (non-deleted) casts
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_cast_timestamp")
                    .table(Casts::Table)
                    .col(Casts::Timestamp)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_cast_parent_hash")
                    .table(Casts::Table)
                    .col(Casts::ParentHash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_cast_root_parent_hash")
                    .table(Casts::Table)
                    .col(Casts::RootParentHash)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_cast_root_parent_url")
                    .table(Casts::Table)
                    .col(Casts::RootParentUrl)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Reactions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Reactions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Reactions::Fid).big_integer().not_null())
                    .col(ColumnDef::new(Reactions::TargetCastFid).big_integer())
                    .col(ColumnDef::new(Reactions::Type).integer().not_null())
                    .col(
                        ColumnDef::new(Reactions::Hash)
                            .text()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Reactions::TargetCastHash).text())
                    .col(ColumnDef::new(Reactions::TargetUrl).text())
                    .col(
                        ColumnDef::new(Reactions::CreateAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .col(
                        ColumnDef::new(Reactions::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .col(
                        ColumnDef::new(Reactions::Timestamp)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Reactions::DeletedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Reactions::PrunedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_reaction_fid_timestamp_deleted_at")
                    .table(Reactions::Table)
                    .col(Reactions::Fid)
                    .col(Reactions::Timestamp)
                    .col(Reactions::DeletedAt) //.nulls_not_distinct()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_reaction_target_cast_hash")
                    .table(Reactions::Table)
                    .col(Reactions::TargetCastHash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_reaction_target_url")
                    .table(Reactions::Table)
                    .col(Reactions::TargetUrl)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Links::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Links::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Links::Fid).big_integer().not_null())
                    .col(ColumnDef::new(Links::TargetFid).big_integer().not_null())
                    .col(ColumnDef::new(Links::DisplayTimestamp).timestamp_with_time_zone())
                    .col(ColumnDef::new(Links::Type).text().not_null())
                    .col(ColumnDef::new(Links::Hash).text().not_null().unique_key())
                    .col(
                        ColumnDef::new(Links::CreateAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .col(
                        ColumnDef::new(Links::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .col(
                        ColumnDef::new(Links::Timestamp)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Links::DeletedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Links::PrunedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Verifications::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Verifications::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Verifications::Fid).big_integer().not_null())
                    .col(ColumnDef::new(Verifications::Hash).text().not_null())
                    .col(
                        ColumnDef::new(Verifications::SignerAddress)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Verifications::BlockHash).text().not_null())
                    .col(ColumnDef::new(Verifications::Signature).text().not_null())
                    .col(
                        ColumnDef::new(Verifications::CreateAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .col(
                        ColumnDef::new(Verifications::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .col(
                        ColumnDef::new(Verifications::Timestamp)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Verifications::DeletedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_verfication_fid_timestamp")
                    .table(Verifications::Table)
                    .col(Verifications::Fid)
                    .col(Verifications::Timestamp)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserData::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserData::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserData::Fid).big_integer().not_null())
                    .col(ColumnDef::new(UserData::Type).integer().not_null())
                    .col(
                        ColumnDef::new(UserData::Hash)
                            .text()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(UserData::Value).text().not_null())
                    .col(
                        ColumnDef::new(UserData::CreateAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .col(
                        ColumnDef::new(UserData::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .col(
                        ColumnDef::new(UserData::Timestamp)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserData::DeletedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Fids::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Fids::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Fids::Fid).big_integer().not_null())
                    .col(
                        ColumnDef::new(Fids::RegisterAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .col(ColumnDef::new(Fids::CustodyAddress).text().not_null())
                    .col(ColumnDef::new(Fids::RecoveryAddress).text().not_null())
                    .col(
                        ColumnDef::new(Fids::CreateAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .col(
                        ColumnDef::new(Fids::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Signers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Signers::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Signers::Fid).big_integer().not_null())
                    .col(
                        ColumnDef::new(Signers::RequesterFid)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Signers::KeyType).big_integer().not_null())
                    .col(ColumnDef::new(Signers::MetadataType).integer().not_null())
                    .col(ColumnDef::new(Signers::Key).text().not_null())
                    .col(ColumnDef::new(Signers::Metadata).json().not_null())
                    .col(
                        ColumnDef::new(Signers::CreateAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .col(
                        ColumnDef::new(Signers::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .col(
                        ColumnDef::new(Signers::AddedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Signers::RemovedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_signer_fix")
                    .table(Signers::Table)
                    .col(Signers::Fid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_signer_requester_fid")
                    .table(Signers::Table)
                    .col(Signers::RequesterFid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Storage::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Storage::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Storage::Fid).big_integer().not_null())
                    .col(ColumnDef::new(Storage::Units).integer().not_null())
                    .col(
                        ColumnDef::new(Storage::Payer)
                            .text()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Storage::CreateAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .col(
                        ColumnDef::new(Storage::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .col(
                        ColumnDef::new(Storage::RentedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Storage::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_storage_fid_expired_at")
                    .table(Storage::Table)
                    .col(Storage::Fid)
                    .col(Storage::ExpiresAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Hubs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Hubs::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Hubs::GossipAddress).text().not_null())
                    .col(ColumnDef::new(Hubs::RpcAddress).text().not_null())
                    .col(
                        ColumnDef::new(Hubs::ExcludedHashes)
                            .array(ColumnType::Text)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Hubs::Count).integer().not_null().default(0))
                    .col(ColumnDef::new(Hubs::HubVersion).text().not_null())
                    .col(ColumnDef::new(Hubs::Network).text().not_null())
                    .col(ColumnDef::new(Hubs::AppVersion).text().not_null())
                    .col(ColumnDef::new(Hubs::Timestamp).integer().not_null())
                    .col(
                        ColumnDef::new(Hubs::CreateAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .col(
                        ColumnDef::new(Hubs::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Casts::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Reactions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Links::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Verifications::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserData::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Fids::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Signers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Storage::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Hubs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Casts {
    Table,
    Id,
    Fid,
    ParentFid,
    Hash,
    RootParentHash,
    ParentHash,
    RootParentUrl,
    ParentUrl,
    Text,
    Embeds,
    Mentions,
    MentionsPositions,

    Timestamp,
    CreateAt,
    UpdatedAt,
    DeletedAt,
    PrunedAt,
}

#[derive(DeriveIden)]
enum Reactions {
    Table,
    Id,
    Fid,
    TargetCastFid,
    Type,
    Hash,
    TargetCastHash,
    TargetUrl,

    Timestamp,
    CreateAt,
    UpdatedAt,
    DeletedAt,
    PrunedAt,
}

#[derive(DeriveIden)]
enum Links {
    Table,
    Id,
    Fid,
    TargetFid,
    DisplayTimestamp,
    Type,
    Hash,

    Timestamp,
    CreateAt,
    UpdatedAt,
    DeletedAt,
    PrunedAt,
}

#[derive(DeriveIden)]
enum Verifications {
    Table,
    Id,
    Fid,
    Hash,
    SignerAddress,
    BlockHash,
    Signature,

    Timestamp,
    CreateAt,
    UpdatedAt,
    DeletedAt,
    // PrunedAt,
}

#[derive(DeriveIden)]
enum UserData {
    Table,
    Id,
    Fid,
    Type,
    Hash,
    Value,

    Timestamp,
    CreateAt,
    UpdatedAt,
    DeletedAt,
    // PrunedAt,
}

#[derive(DeriveIden)]
enum Fids {
    Table,
    Id, // why not
    Fid,
    RegisterAt,
    CustodyAddress,
    RecoveryAddress,

    CreateAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Signers {
    Table,
    Id,
    Fid,
    RequesterFid,
    KeyType,
    MetadataType,
    Key,
    Metadata,

    CreateAt,
    UpdatedAt,
    AddedAt,
    RemovedAt,
}

#[derive(DeriveIden)]
enum Storage {
    Table,
    Id,
    Fid,
    Units,
    Payer,

    CreateAt,
    UpdatedAt,
    RentedAt,
    ExpiresAt,
}

#[derive(DeriveIden)]
enum Hubs {
    Table,
    Id,
    GossipAddress,
    RpcAddress,
    ExcludedHashes,
    Count,
    HubVersion,
    Network,
    AppVersion,

    Timestamp,
    CreateAt,
    UpdatedAt,
}
