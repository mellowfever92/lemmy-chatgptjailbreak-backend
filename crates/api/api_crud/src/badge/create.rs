use activitypub_federation::config::Data;
use actix_web::web::Json;
use lemmy_api_utils::{context::LemmyContext, utils::is_admin};
use lemmy_db_schema::source::badge::{Badge, BadgeInsertForm};
use lemmy_db_views_badge::{BadgeView, api::{CreateBadge, BadgeResponse}};
use lemmy_db_views_local_user::LocalUserView;
use lemmy_diesel_utils::traits::Crud;
use lemmy_utils::error::LemmyResult;

pub async fn create_badge(
  Json(data): Json<CreateBadge>,
  context: Data<LemmyContext>,
  local_user_view: LocalUserView,
) -> LemmyResult<Json<BadgeResponse>> {
  // Make sure user is an admin
  is_admin(&local_user_view)?;

  let badge_form = BadgeInsertForm {
    name: data.name.trim().to_string(),
    description: data.description.map(|d| d.trim().to_string()),
    image_url: data.image_url.clone(),
    is_assignable_by_mods: data.is_assignable_by_mods,
    is_self_selectable: data.is_self_selectable,
  };
  let badge = Badge::create(&mut context.pool(), &badge_form).await?;

  Ok(Json(BadgeResponse {
    badge: BadgeView::from_badge(badge),
  }))
}
