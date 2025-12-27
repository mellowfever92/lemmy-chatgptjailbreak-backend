use activitypub_federation::config::Data;
use actix_web::web::Json;
use lemmy_api_utils::{context::LemmyContext, utils::is_admin};
use lemmy_db_schema::source::badge::Badge;
use lemmy_db_views_badge::api::{DeleteBadge, BadgeActionResponse};
use lemmy_db_views_local_user::LocalUserView;
use lemmy_utils::error::LemmyResult;

pub async fn delete_badge(
  Json(data): Json<DeleteBadge>,
  context: Data<LemmyContext>,
  local_user_view: LocalUserView,
) -> LemmyResult<Json<BadgeActionResponse>> {
  // Make sure user is an admin
  is_admin(&local_user_view)?;

  Badge::delete(&mut context.pool(), data.id).await?;

  Ok(Json(BadgeActionResponse { success: true }))
}
