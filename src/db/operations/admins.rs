use crate::db::db::Pool;
use crate::db::operations::internal;
use crate::rules::engine::*;
use crate::rules::rules::RuleContext;
use crate::user::User;
use dino_park_gate::scope::ScopeAndUser;
use failure::Error;

pub fn add_admin(
    pool: &Pool,
    scope_and_user: &ScopeAndUser,
    group_name: &str,
    host: User,
    user: User,
) -> Result<(), Error> {
    HOST_IS_GROUP_ADMIN.run(&RuleContext::minimal(
        pool,
        scope_and_user,
        &group_name,
        &host.user_uuid,
    ))?;
    let group = internal::group::get_group(pool, group_name)?;
    internal::admin::add_admin(pool, group.id, user.user_uuid, host.user_uuid).map(|_| ())
}
