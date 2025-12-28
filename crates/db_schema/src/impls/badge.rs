use crate::{
  newtypes::BadgeId,
  source::badge::{Badge, BadgeInsertForm, BadgeUpdateForm, PersonBadge, PersonBadgeInsertForm},
};
use lemmy_db_schema_file::PersonId;
use diesel::{dsl::insert_into, ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use lemmy_db_schema_file::schema::{
  badge::dsl::{badge, id as badge_id_col},
  person_badge::dsl::{badge_id, person_badge, person_id},
};
use lemmy_diesel_utils::{
  connection::{get_conn, DbPool},
  traits::Crud,
};
use lemmy_utils::error::{LemmyErrorExt, LemmyErrorType, LemmyResult};

impl Crud for Badge {
  type InsertForm = BadgeInsertForm;
  type UpdateForm = BadgeUpdateForm;
  type IdType = BadgeId;

  async fn create(pool: &mut DbPool<'_>, form: &Self::InsertForm) -> LemmyResult<Self> {
    let conn = &mut get_conn(pool).await?;
    insert_into(badge)
      .values(form)
      .get_result::<Self>(conn)
      .await
      .with_lemmy_type(LemmyErrorType::CouldntCreate)
  }

  async fn update(
    pool: &mut DbPool<'_>,
    id: Self::IdType,
    new_badge: &Self::UpdateForm,
  ) -> LemmyResult<Self> {
    let conn = &mut get_conn(pool).await?;
    diesel::update(badge.filter(badge_id_col.eq(id)))
      .set(new_badge)
      .get_result::<Self>(conn)
      .await
      .with_lemmy_type(LemmyErrorType::CouldntUpdate)
  }
}

impl Badge {
  pub async fn read(pool: &mut DbPool<'_>, id: BadgeId) -> LemmyResult<Self> {
    let conn = &mut get_conn(pool).await?;
    badge
      .filter(badge_id_col.eq(id))
      .first::<Self>(conn)
      .await
      .with_lemmy_type(LemmyErrorType::CouldntUpdate)
  }

  pub async fn delete(pool: &mut DbPool<'_>, id: BadgeId) -> LemmyResult<usize> {
    let conn = &mut get_conn(pool).await?;
    diesel::delete(badge.filter(badge_id_col.eq(id)))
      .execute(conn)
      .await
      .with_lemmy_type(LemmyErrorType::CouldntUpdate)
  }

  pub async fn list(pool: &mut DbPool<'_>) -> LemmyResult<Vec<Self>> {
    let conn = &mut get_conn(pool).await?;
    badge
      .order(badge_id_col.asc())
      .load::<Self>(conn)
      .await
      .with_lemmy_type(LemmyErrorType::CouldntUpdate)
  }
}

impl PersonBadge {
  pub async fn create(pool: &mut DbPool<'_>, form: &PersonBadgeInsertForm) -> LemmyResult<Self> {
    let conn = &mut get_conn(pool).await?;
    insert_into(person_badge)
      .values(form)
      .get_result::<Self>(conn)
      .await
      .with_lemmy_type(LemmyErrorType::CouldntCreate)
  }

  pub async fn delete(
    pool: &mut DbPool<'_>,
    for_person_id: PersonId,
    for_badge_id: BadgeId,
  ) -> LemmyResult<usize> {
    let conn = &mut get_conn(pool).await?;
    diesel::delete(
      person_badge
        .filter(person_id.eq(for_person_id))
        .filter(badge_id.eq(for_badge_id)),
    )
    .execute(conn)
    .await
    .with_lemmy_type(LemmyErrorType::CouldntUpdate)
  }

  pub async fn list_for_person(
    pool: &mut DbPool<'_>,
    for_person_id: PersonId,
  ) -> LemmyResult<Vec<Self>> {
    let conn = &mut get_conn(pool).await?;
    person_badge
      .filter(person_id.eq(for_person_id))
      .load::<Self>(conn)
      .await
      .with_lemmy_type(LemmyErrorType::CouldntUpdate)
  }
}
