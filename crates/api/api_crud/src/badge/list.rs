use activitypub_federation::config::Data;
use actix_web::web::Json;
use lemmy_api_utils::context::LemmyContext;
use lemmy_db_schema::source::badge::Badge;
use lemmy_db_views_badge::{BadgeView, api::{ListBadges, ListBadgesResponse}};
use lemmy_utils::error::LemmyResult;

pub async fn list_badges(
  _data: Json<ListBadges>,
  context: Data<LemmyContext>,
) -> LemmyResult<Json<ListBadgesResponse>> {
  let badges = Badge::list(&mut context.pool()).await?;

  Ok(Json(ListBadgesResponse {
    badges: badges.into_iter().map(BadgeView::from_badge).collect(),
  }))
}
