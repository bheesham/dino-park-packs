use crate::db::schema::*;
use crate::db::types::*;
use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Identifiable, Queryable, PartialEq, Debug, AsChangeset, Serialize)]
#[table_name = "groups"]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub active: bool,
    pub path: String,
    pub description: String,
    pub capabilities: Vec<CapabilityType>,
    pub typ: GroupType,
    pub trust: TrustType,
    pub group_expiration: Option<i32>,
    pub created: NaiveDateTime,
}

#[derive(Identifiable, Associations, Queryable, PartialEq, Debug, Insertable, AsChangeset)]
#[belongs_to(Group)]
#[primary_key(group_id)]
#[table_name = "terms"]
pub struct Terms {
    pub group_id: i32,
    pub text: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
pub struct Role {
    pub id: i32,
    pub group_id: i32,
    pub typ: RoleType,
    pub name: String,
    pub permissions: Vec<PermissionType>,
}

#[derive(Queryable, Associations, PartialEq, Debug, Insertable, AsChangeset)]
#[belongs_to(Group)]
#[primary_key(group_id, user_uuid)]
pub struct Membership {
    pub user_uuid: Uuid,
    pub group_id: i32,
    pub role_id: i32,
    pub expiration: Option<NaiveDateTime>,
    pub added_by: Uuid,
    pub added_ts: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Insertable, AsChangeset)]
#[belongs_to(Group)]
#[primary_key(group_id, user_uuid)]
pub struct Invitation {
    pub group_id: i32,
    pub user_uuid: Uuid,
    pub invitation_expiration: Option<NaiveDateTime>,
    pub group_expiration: Option<i32>,
    pub added_by: Uuid,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Insertable, AsChangeset)]
#[belongs_to(Group)]
#[primary_key(group_id, user_uuid)]
pub struct Request {
    pub group_id: i32,
    pub user_uuid: Uuid,
    pub created: NaiveDateTime,
    pub request_expiration: Option<NaiveDateTime>,
    pub msg: Option<String>,
}

#[derive(Queryable, Serialize)]
pub struct GroupsList {
    pub name: String,
    pub typ: GroupType,
    pub trust: TrustType,
    pub member_count: i64,
}

#[derive(Insertable)]
#[table_name = "groups"]
pub struct InsertGroup {
    pub name: String,
    pub active: bool,
    pub path: String,
    pub description: String,
    pub capabilities: Vec<CapabilityType>,
    pub typ: GroupType,
    pub trust: TrustType,
    pub group_expiration: Option<i32>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "memberships"]
#[changeset_options(treat_none_as_null = "true")]
pub struct InsertMembership {
    pub user_uuid: Uuid,
    pub group_id: i32,
    pub role_id: i32,
    pub expiration: Option<NaiveDateTime>,
    pub added_by: Uuid,
}

#[derive(Insertable)]
#[table_name = "roles"]
pub struct InsertRole {
    pub group_id: i32,
    pub typ: RoleType,
    pub name: String,
    pub permissions: Vec<PermissionType>,
}

#[derive(Insertable)]
#[table_name = "requests"]
pub struct InsertRequest {
    pub group_id: i32,
    pub user_uuid: Uuid,
    pub request_expiration: Option<NaiveDateTime>,
}
