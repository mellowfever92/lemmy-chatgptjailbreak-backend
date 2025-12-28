use actix_web::{web::*, HttpResponse};
use lemmy_api_utils::context::LemmyContext;
use lemmy_db_schema::source::badge::{
  Badge,
  BadgeInsertForm,
  BadgeUpdateForm,
  PersonBadge,
  PersonBadgeInsertForm,
};
use lemmy_db_views_badge::{
  api::{
    AssignBadge,
    AssignedBadge,
    BadgeActionResponse,
    BadgeResponse,
    CreateBadge,
    DeleteBadge,
    EditBadge,
    ListBadgesResponse,
    PersonBadgesResponse,
    RemoveBadge,
  },
  BadgeView,
};
use lemmy_db_views_local_user::LocalUserView;
use lemmy_diesel_utils::traits::Crud;
use lemmy_utils::{error::LemmyResult, error::LemmyErrorType};

pub(crate) async fn list_badges_v3(
  context: Data<LemmyContext>,
) -> LemmyResult<Json<ListBadgesResponse>> {
  let badges = BadgeView::list(&mut context.pool()).await?;
  Ok(Json(ListBadgesResponse { badges }))
}

pub(crate) async fn create_badge_v3(
  data: Json<CreateBadge>,
  context: Data<LemmyContext>,
  local_user_view: LocalUserView,
) -> LemmyResult<Json<BadgeResponse>> {
  // Require admin
  if !local_user_view.local_user.admin {
    Err(LemmyErrorType::NotAnAdmin)?
  }

  let form = BadgeInsertForm {
    name: data.name.clone(),
    description: data.description.clone(),
    image_url: data.image_url.clone(),
    is_assignable_by_mods: data.is_assignable_by_mods,
    is_self_selectable: data.is_self_selectable,
  };

  let badge = Badge::create(&mut context.pool(), &form).await?;
  let badge_view = BadgeView::read(&mut context.pool(), badge.id).await?;

  Ok(Json(BadgeResponse { badge: badge_view }))
}

pub(crate) async fn update_badge_v3(
  data: Json<EditBadge>,
  context: Data<LemmyContext>,
  local_user_view: LocalUserView,
) -> LemmyResult<Json<BadgeResponse>> {
  // Require admin
  if !local_user_view.local_user.admin {
    Err(LemmyErrorType::NotAnAdmin)?
  }

  let form = BadgeUpdateForm {
    name: data.name.clone(),
    description: data.description.clone(),
    image_url: data.image_url.clone(),
    is_assignable_by_mods: data.is_assignable_by_mods,
    is_self_selectable: data.is_self_selectable,
  };

  let badge = Badge::update(&mut context.pool(), data.id, &form).await?;
  let badge_view = BadgeView::read(&mut context.pool(), badge.id).await?;

  Ok(Json(BadgeResponse { badge: badge_view }))
}

pub(crate) async fn delete_badge_v3(
  data: Json<DeleteBadge>,
  context: Data<LemmyContext>,
  local_user_view: LocalUserView,
) -> LemmyResult<HttpResponse> {
  // Require admin
  if !local_user_view.local_user.admin {
    Err(LemmyErrorType::NotAnAdmin)?
  }

  Badge::delete(&mut context.pool(), data.id).await?;

  Ok(HttpResponse::Ok().json(BadgeActionResponse { success: true }))
}

pub(crate) async fn assign_badge_v3(
  data: Json<AssignBadge>,
  context: Data<LemmyContext>,
  local_user_view: LocalUserView,
) -> LemmyResult<HttpResponse> {
  // Require admin (TODO: or mod if badge is assignable_by_mods)
  if !local_user_view.local_user.admin {
    Err(LemmyErrorType::NotAnAdmin)?
  }

  let form = PersonBadgeInsertForm {
    person_id: data.person_id,
    badge_id: data.badge_id,
    assigned_by: Some(local_user_view.person.id),
  };

  PersonBadge::create(&mut context.pool(), &form).await?;

  Ok(HttpResponse::Ok().json(BadgeActionResponse { success: true }))
}

pub(crate) async fn remove_badge_v3(
  data: Json<RemoveBadge>,
  context: Data<LemmyContext>,
  local_user_view: LocalUserView,
) -> LemmyResult<HttpResponse> {
  // Require admin (TODO: or mod if badge is assignable_by_mods)
  if !local_user_view.local_user.admin {
    Err(LemmyErrorType::NotAnAdmin)?
  }

  PersonBadge::delete(&mut context.pool(), data.person_id, data.badge_id).await?;

  Ok(HttpResponse::Ok().json(BadgeActionResponse { success: true }))
}

pub(crate) async fn get_person_badges_v3(
  person_id: Path<i32>,
  context: Data<LemmyContext>,
) -> LemmyResult<Json<PersonBadgesResponse>> {
  let badges = BadgeView::list_for_person(&mut context.pool(), (*person_id).into()).await?;
  
  let assigned_badges: Vec<AssignedBadge> = badges.into_iter().map(|(badge_view, person_badge)| {
    AssignedBadge {
      badge: badge_view,
      assigned_at: person_badge.assigned_at.to_rfc3339(),
      assigned_by: person_badge.assigned_by,
    }
  }).collect();

  Ok(Json(PersonBadgesResponse { badges: assigned_badges }))
}
