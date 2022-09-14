//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use super::sea_orm_active_enums::Status;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "order")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub serial_order: Option<String>,
    pub production_id: Option<u32>,
    pub customer_id: Option<u32>,
    pub count: Option<i32>,
    pub price: Option<f32>,
    pub old_price: Option<f32>,
    pub profits: Option<f32>,
    pub owner_id: Option<u32>,
    pub money: Option<f32>,
    pub settled_money: Option<f32>,
    pub invoice_status: Option<f32>,
    pub status: Option<Status>,
    pub take_goods: Option<u32>,
    pub created_at: DateTimeUtc,
    pub update_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::production::Entity",
        from = "Column::ProductionId",
        to = "super::production::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Production,
    #[sea_orm(
        belongs_to = "super::customs::Entity",
        from = "Column::CustomerId",
        to = "super::customs::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Customs,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::OwnerId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::goods_info::Entity",
        from = "Column::TakeGoods",
        to = "super::goods_info::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    GoodsInfo,
}

impl Related<super::production::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Production.def()
    }
}

impl Related<super::customs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customs.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::goods_info::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GoodsInfo.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
