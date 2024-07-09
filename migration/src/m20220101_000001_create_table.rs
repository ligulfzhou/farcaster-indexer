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
                    .col(ColumnDef::new(Casts::ParentId).big_integer())
                    .col(ColumnDef::new(Casts::Hash).text().not_null().unique_key())
                    .col(ColumnDef::new(Casts::RootParentHash).text())
                    .col(ColumnDef::new(Casts::ParentHash).text())
                    .col(ColumnDef::new(Casts::RootParentUrl).text())
                    .col(ColumnDef::new(Casts::ParentUrl).text())
                    .col(ColumnDef::new(Casts::Text).text().not_null())
                    .col(ColumnDef::new(Casts::Embeds).json().not_null().default("[]"))
                    .col(ColumnDef::new(Casts::Mentions).json().not_null().default("[]"))
                    .col(ColumnDef::new(Casts::MentionsPositions).json().not_null().default("[]"))

                    .col(ColumnDef::new(Casts::CreateAt).timestamp_with_time_zone().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .col(ColumnDef::new(Casts::UpdatedAt).timestamp_with_time_zone().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .col(ColumnDef::new(Casts::Timestamp).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Casts::DeletedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Casts::PrunedAt).timestamp_with_time_zone())
                    .to_owned(),
            ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Reaction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Reaction::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Reaction::Fid).big_integer().not_null())
                    .col(ColumnDef::new(Reaction::TargetCastsFid).big_integer())
                    .col(ColumnDef::new(Reaction::Type).integer().not_null())
                    .col(ColumnDef::new(Reaction::Hash).text().not_null().unique_key())
                    .col(ColumnDef::new(Reaction::TargetCastsHash).text())
                    .col(ColumnDef::new(Reaction::TargetUrl).text())

                    .col(ColumnDef::new(Reaction::CreateAt).timestamp_with_time_zone().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .col(ColumnDef::new(Reaction::UpdatedAt).timestamp_with_time_zone().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .col(ColumnDef::new(Reaction::Timestamp).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Reaction::DeletedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Reaction::PrunedAt).timestamp_with_time_zone())
                    .to_owned(),
            ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Link::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Link::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Link::Fid).big_integer().not_null())
                    .col(ColumnDef::new(Link::TargetFid).big_integer().not_null())
                    .col(ColumnDef::new(Link::DisplayTimestamp).timestamp_with_time_zone())
                    .col(ColumnDef::new(Link::Type).text().not_null())
                    .col(ColumnDef::new(Link::Hash).text().not_null().unique_key())

                    .col(ColumnDef::new(Link::CreateAt).timestamp_with_time_zone().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .col(ColumnDef::new(Link::UpdatedAt).timestamp_with_time_zone().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .col(ColumnDef::new(Link::Timestamp).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Link::DeletedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Link::PrunedAt).timestamp_with_time_zone())
                    .to_owned(),
            ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Verification::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Verification::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Verification::Fid).big_integer().not_null())
                    .col(ColumnDef::new(Verification::Hash).text().not_null())
                    .col(ColumnDef::new(Verification::SignerAddress).text().not_null())
                    .col(ColumnDef::new(Verification::BlockHash).text().not_null())
                    .col(ColumnDef::new(Verification::Signature).text().not_null())

                    .col(ColumnDef::new(Verification::CreateAt).timestamp_with_time_zone().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .col(ColumnDef::new(Verification::UpdatedAt).timestamp_with_time_zone().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .col(ColumnDef::new(Verification::Timestamp).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Verification::DeletedAt).timestamp_with_time_zone())
                    .to_owned(),
            ).await?;

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
                    .col(ColumnDef::new(UserData::Hash).text().not_null().unique_key())
                    .col(ColumnDef::new(UserData::Value).text().not_null())

                    .col(ColumnDef::new(UserData::CreateAt).timestamp_with_time_zone().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .col(ColumnDef::new(UserData::UpdatedAt).timestamp_with_time_zone().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .col(ColumnDef::new(UserData::Timestamp).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(UserData::DeletedAt).timestamp_with_time_zone())
                    .to_owned(),
            ).await?;


        manager
            .create_table(
                Table::create()
                    .table(Fid::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Fid::RegisterAt).timestamp_with_time_zone().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .col(ColumnDef::new(Fid::CustodyAddress).text().not_null())
                    .col(ColumnDef::new(Fid::RecoveryAddress).text().not_null())

                    .col(ColumnDef::new(Fid::CreateAt).timestamp_with_time_zone().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .col(ColumnDef::new(Fid::UpdatedAt).timestamp_with_time_zone().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Signer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Signer::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Signer::Fid).big_integer().not_null())
                    .col(ColumnDef::new(Signer::RequesterFid).big_integer().not_null())
                    .col(ColumnDef::new(Signer::KeyType).big_integer().not_null())
                    .col(ColumnDef::new(Signer::MetadataType).integer().not_null())
                    .col(ColumnDef::new(Signer::Key).text().not_null())
                    .col(ColumnDef::new(Signer::Metadata).json().not_null())

                    .col(ColumnDef::new(Signer::CreateAt).timestamp_with_time_zone().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .col(ColumnDef::new(Signer::UpdatedAt).timestamp_with_time_zone().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .col(ColumnDef::new(Signer::AddedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Signer::RemovedAt).timestamp_with_time_zone())
                    .to_owned(),
            ).await?;

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
                    .col(ColumnDef::new(Storage::Payer).text().not_null().unique_key())

                    .col(ColumnDef::new(Storage::CreateAt).timestamp_with_time_zone().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .col(ColumnDef::new(Storage::UpdatedAt).timestamp_with_time_zone().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .col(ColumnDef::new(Storage::RentedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Storage::ExpiresAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Hub::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Hub::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Hub::GossipAddress).text().not_null())
                    .col(ColumnDef::new(Hub::RpcAddress).text().not_null())
                    .col(ColumnDef::new(Hub::ExcludedHashes).array(ColumnType::Text).not_null())
                    .col(ColumnDef::new(Hub::Count).integer().not_null().default(0))
                    .col(ColumnDef::new(Hub::HubVersion).text().not_null())
                    .col(ColumnDef::new(Hub::Network).text().not_null())
                    .col(ColumnDef::new(Hub::AppVersion).text().not_null())
                    .col(ColumnDef::new(Hub::Timestamp).integer().not_null())

                    .col(ColumnDef::new(Hub::CreateAt).timestamp_with_time_zone().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .col(ColumnDef::new(Hub::UpdatedAt).timestamp_with_time_zone().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .to_owned(),
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Casts::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Reaction::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Link::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Verification::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserData::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Fid::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Signer::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Storage::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Hub::Table).to_owned())
            .await
    }
}


#[derive(DeriveIden)]
enum Casts {
    Table,
    Id,
    Fid,
    ParentId,
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
enum Reaction {
    Table,
    Id,
    Fid,
    TargetCastsFid,
    Type,
    Hash,
    TargetCastsHash,
    TargetUrl,

    Timestamp,
    CreateAt,
    UpdatedAt,
    DeletedAt,
    PrunedAt,
}


#[derive(DeriveIden)]
enum Link {
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
enum Verification {
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
enum Fid {
    Table,
    // Id, // why not
    Fid,
    RegisterAt,
    CustodyAddress,
    RecoveryAddress,

    CreateAt,
    UpdatedAt,
}


#[derive(DeriveIden)]
enum Signer {
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
    ExpiresAt
}

#[derive(DeriveIden)]
enum Hub {
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
