use crate::db::internal;
use crate::db::internal::request::*;
use crate::db::operations::models::*;
use crate::db::Pool;
use crate::mail::manager::send_email;
use crate::mail::manager::send_emails;
use crate::mail::templates::Template;
use crate::rules::engine::*;
use crate::rules::RuleContext;
use crate::user::User;
use chrono::Duration;
use chrono::NaiveDateTime;
use chrono::Utc;
use dino_park_gate::scope::ScopeAndUser;
use dino_park_trust::Trust;
use failure::Error;

pub fn request_membership(
    pool: &Pool,
    scope_and_user: &ScopeAndUser,
    group_name: &str,
    request_expiration: Option<NaiveDateTime>,
) -> Result<(), Error> {
    let mut connection = pool.get()?;
    let user = internal::user::user_by_id(&mut connection, &scope_and_user.user_id)?;
    CURRENT_USER_CAN_REQUEST.run(&RuleContext::minimal(
        pool,
        scope_and_user,
        group_name,
        &user.user_uuid,
    ))?;
    request(&mut connection, group_name, user, request_expiration)
}

pub fn reject_request(
    pool: &Pool,
    scope_and_user: &ScopeAndUser,
    group_name: &str,
    user: &User,
) -> Result<(), Error> {
    let mut connection = pool.get()?;
    let host = internal::user::user_by_id(&mut connection, &scope_and_user.user_id)?;
    HOST_IS_CURATOR.run(&RuleContext::minimal(
        pool,
        scope_and_user,
        group_name,
        &host.user_uuid,
    ))?;
    reject(&mut connection, group_name, &host, user)?;
    let p = internal::user::slim_user_profile_by_uuid(&mut connection, &user.user_uuid)?;
    send_email(p.email, &Template::RejectRequest(group_name.to_owned()));
    Ok(())
}

pub fn cancel_request(
    pool: &Pool,
    scope_and_user: &ScopeAndUser,
    group_name: &str,
) -> Result<(), Error> {
    let mut connection = pool.get()?;
    let user = internal::user::user_by_id(&mut connection, &scope_and_user.user_id)?;
    cancel(&mut connection, group_name, &user)
}

pub fn request_count(
    pool: &Pool,
    scope_and_user: &ScopeAndUser,
    group_name: &str,
) -> Result<i64, Error> {
    let mut connection = pool.get()?;
    let user = internal::user::user_by_id(&mut connection, &scope_and_user.user_id)?;
    HOST_IS_CURATOR.run(&RuleContext::minimal(
        pool,
        scope_and_user,
        group_name,
        &user.user_uuid,
    ))?;
    count(&mut connection, group_name)
}

pub fn pending_requests_for_user(
    pool: &Pool,
    scope_and_user: &ScopeAndUser,
) -> Result<Vec<DisplayRequestForUser>, Error> {
    let mut connection = pool.get()?;
    let user = internal::user::user_by_id(&mut connection, &scope_and_user.user_id)?;
    requests_for_user(&mut connection, &user)
}

pub fn pending_requests(
    pool: &Pool,
    scope_and_user: &ScopeAndUser,
    group_name: &str,
) -> Result<Vec<DisplayRequest>, Error> {
    let mut connection = pool.get()?;
    let user = internal::user::user_by_id(&mut connection, &scope_and_user.user_id)?;
    HOST_IS_CURATOR.run(&RuleContext::minimal(
        pool,
        scope_and_user,
        group_name,
        &user.user_uuid,
    ))?;
    match scope_and_user.scope {
        Trust::Staff => staff_scoped_requests(&mut connection, group_name),
        Trust::Ndaed => ndaed_scoped_requests(&mut connection, group_name),
        Trust::Vouched => vouched_scoped_requests(&mut connection, group_name),
        Trust::Authenticated => authenticated_scoped_requests(&mut connection, group_name),
        Trust::Public => public_scoped_requests(&mut connection, group_name),
    }
}

pub fn pending_requests_notification(pool: &Pool) -> Result<(), Error> {
    let lower = Utc::now()
        .checked_sub_signed(Duration::days(1))
        .unwrap()
        .date()
        .and_hms(0, 0, 0)
        .naive_utc();
    let upper = Utc::now()
        .checked_sub_signed(Duration::days(1))
        .unwrap()
        .date()
        .and_hms_nano(23, 59, 59, 999_999_999)
        .naive_utc();
    let mut connection = pool.get()?;
    let pending = internal::request::new_pending(&mut connection, lower, upper)?;
    for (group_id, npr) in pending {
        let bcc = internal::member::get_curator_emails(&mut connection, group_id)?;
        send_emails(bcc, &Template::PendingRequest(npr.group_name, npr.count));
    }
    Ok(())
}
