use activitypub_federation::config::Data;
use actix_web::web::Json;
use lemmy_api_utils::{context::LemmyContext, utils::is_admin};
use lemmy_db_schema::source::badge::{PersonBadge, PersonBadgeInsertForm};
use lemmy_db_views_badge::api::{AssignBadge, BadgeActionResponse};
use lemmy_db_views_local_user::LocalUserView;
use lemmy_utils::error::LemmyResult;

pub async fn assign_badge(
  Json(data): Json<AssignBadge>,
  context: Data<LemmyContext>,
  local_user_view: LocalUserView,
) -> LemmyResult<Json<BadgeActionResponse>> {
  // Make sure user is an admin (can be extended to check for mod privileges)
  is_admin(&local_user_view)?;

  let badge_form = PersonBadgeInsertForm {
    person_id: data.person_id,
    badge_id: data.badge_id,
    assigned_by: Some(local_user_view.person.id),
  };
  PersonBadge::create(&mut context.pool(), &badge_form).await?;

  Ok(Json(BadgeActionResponse { success: true }))
}
