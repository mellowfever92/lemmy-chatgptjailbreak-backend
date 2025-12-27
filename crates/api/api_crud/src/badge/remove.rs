use activitypub_federation::config::Data;
use actix_web::web::Json;
use lemmy_api_utils::{context::LemmyContext, utils::is_admin};
use lemmy_db_schema::source::badge::PersonBadge;
use lemmy_db_views_badge::api::{RemoveBadge, BadgeActionResponse};
use lemmy_db_views_local_user::LocalUserView;
use lemmy_utils::error::LemmyResult;

pub async fn remove_badge(
  Json(data): Json<RemoveBadge>,
  context: Data<LemmyContext>,
  local_user_view: LocalUserView,
) -> LemmyResult<Json<BadgeActionResponse>> {
  // Make sure user is an admin (can be extended to check for mod privileges)
  is_admin(&local_user_view)?;

  PersonBadge::delete(&mut context.pool(), data.person_id, data.badge_id).await?;

  Ok(Json(BadgeActionResponse { success: true }))
}
