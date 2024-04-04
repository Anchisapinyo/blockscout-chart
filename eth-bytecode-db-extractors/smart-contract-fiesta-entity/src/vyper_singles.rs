//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "vyper_singles")]
pub struct Model {
    #[sea_orm(
        primary_key,
        auto_increment = false,
        column_type = "Binary(BlobSize::Blob(None))"
    )]
    pub contract_address: Vec<u8>,
    pub contract_name: String,
    pub compiler_version: String,
    pub optimizations: bool,
    pub optimization_runs: i64,
    pub source_code: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::contract_addresses::Entity",
        from = "Column::ContractAddress",
        to = "super::contract_addresses::Column::ContractAddress",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ContractAddresses,
}

impl Related<super::contract_addresses::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ContractAddresses.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
