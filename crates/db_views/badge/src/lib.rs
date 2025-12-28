use lemmy_db_schema::{newtypes::BadgeId, source::badge::Badge};
use lemmy_db_schema_file::schema::badge as badge_schema;
use lemmy_diesel_utils::connection::{DbPool, get_conn};
use lemmy_utils::error::LemmyResult;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

pub mod api;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(optional_fields, export))]
/// A badge view (same as Badge for now, but allows for future expansion).
pub struct BadgeView {
  #[serde(flatten)]
  pub badge: Badge,
}

impl BadgeView {
  pub fn from_badge(badge: Badge) -> Self {
    Self { badge }
  }

  pub async fn list(pool: &mut DbPool<'_>) -> LemmyResult<Vec<Self>> {
    use badge_schema::dsl::*;
    
    let conn = &mut get_conn(pool).await?;
    let badge_list = badge
      .order_by(name.asc())
      .load::<Badge>(conn)
      .await?;
      
    Ok(badge_list.into_iter().map(Self::from_badge).collect())
  }

  pub async fn read(pool: &mut DbPool<'_>, badge_id: BadgeId) -> LemmyResult<Self> {
    use badge_schema::dsl::*;
    
    let conn = &mut get_conn(pool).await?;
    let badge_obj = badge
      .filter(id.eq(badge_id))
      .first::<Badge>(conn)
      .await?;
      
    Ok(Self::from_badge(badge_obj))
  }
}

