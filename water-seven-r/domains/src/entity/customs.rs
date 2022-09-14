//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "customs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String,
    pub phone: Option<String>,
    pub group_org: Option<u32>,
    pub owner_id: Option<u32>,
    pub outside_money: Option<f32>,
    pub discount: Option<f32>,
    pub created_at: DateTimeUtc,
    pub update_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::customs_org::Entity",
        from = "Column::GroupOrg",
        to = "super::customs_org::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    CustomsOrg,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::OwnerId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
    #[sea_orm(has_many = "super::order::Entity")]
    Order,
}

impl Related<super::customs_org::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CustomsOrg.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::order::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Order.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
