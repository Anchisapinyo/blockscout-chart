//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "token_transfers")]
pub struct Model {
    #[sea_orm(
        primary_key,
        auto_increment = false,
        column_type = "Binary(BlobSize::Blob(None))"
    )]
    pub transaction_hash: Vec<u8>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub log_index: i32,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub from_address_hash: Vec<u8>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub to_address_hash: Vec<u8>,
    pub amount: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((78, 0)))", nullable)]
    pub token_id: Option<Decimal>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub token_contract_address_hash: Vec<u8>,
    pub inserted_at: DateTime,
    pub updated_at: DateTime,
    pub block_number: Option<i32>,
    #[sea_orm(
        primary_key,
        auto_increment = false,
        column_type = "Binary(BlobSize::Blob(None))"
    )]
    pub block_hash: Vec<u8>,
    pub amounts: Option<Vec<Decimal>>,
    pub token_ids: Option<Vec<Decimal>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::addresses::Entity",
        from = "Column::FromAddressHash",
        to = "super::addresses::Column::Hash",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Addresses3,
    #[sea_orm(
        belongs_to = "super::addresses::Entity",
        from = "Column::ToAddressHash",
        to = "super::addresses::Column::Hash",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Addresses2,
    #[sea_orm(
        belongs_to = "super::addresses::Entity",
        from = "Column::TokenContractAddressHash",
        to = "super::addresses::Column::Hash",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Addresses1,
    #[sea_orm(
        belongs_to = "super::blocks::Entity",
        from = "Column::BlockHash",
        to = "super::blocks::Column::Hash",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Blocks,
    #[sea_orm(
        belongs_to = "super::transactions::Entity",
        from = "Column::TransactionHash",
        to = "super::transactions::Column::Hash",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Transactions,
}

impl Related<super::blocks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Blocks.def()
    }
}

impl Related<super::transactions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Transactions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}