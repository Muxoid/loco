//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "roles_name")]
pub enum RolesName {
    #[sea_orm(string_value = "Admin")]
    Admin,
    #[sea_orm(string_value = "User")]
    User,
}
