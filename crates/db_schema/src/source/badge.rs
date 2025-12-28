use crate::newtypes::{BadgeId, PersonBadgeId};
use lemmy_db_schema_file::PersonId;
use chrono::{DateTime, Utc};
#[cfg(feature = "full")]
use lemmy_db_schema_file::schema::{badge, person_badge};
use lemmy_diesel_utils::dburl::DbUrl;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Selectable, Identifiable))]
#[cfg_attr(feature = "full", diesel(table_name = badge))]
#[cfg_attr(feature = "full", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(optional_fields, export))]
/// A badge definition.
pub struct Badge {
  pub id: BadgeId,
  pub name: String,
  pub description: Option<String>,
  pub image_url: DbUrl,
  pub is_assignable_by_mods: bool,
  pub is_self_selectable: bool,
  pub published: DateTime<Utc>,
  pub updated: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = badge))]
pub struct BadgeInsertForm {
  pub name: String,
  pub description: Option<String>,
  pub image_url: DbUrl,
  pub is_assignable_by_mods: bool,
  pub is_self_selectable: bool,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = badge))]
pub struct BadgeUpdateForm {
  pub name: Option<String>,
  pub description: Option<Option<String>>,
  pub image_url: Option<DbUrl>,
  pub is_assignable_by_mods: Option<bool>,
  pub is_self_selectable: Option<bool>,
}

#[skip_serializing_none]
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Selectable, Identifiable, Associations))]
#[cfg_attr(feature = "full", diesel(belongs_to(crate::source::person::Person)))]
#[cfg_attr(feature = "full", diesel(belongs_to(Badge)))]
#[cfg_attr(feature = "full", diesel(table_name = person_badge))]
#[cfg_attr(feature = "full", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(optional_fields, export))]
/// A person's badge assignment.
pub struct PersonBadge {
  pub id: PersonBadgeId,
  pub person_id: PersonId,
  pub badge_id: BadgeId,
  pub assigned_at: DateTime<Utc>,
  pub assigned_by: Option<PersonId>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = person_badge))]
pub struct PersonBadgeInsertForm {
  pub person_id: PersonId,
  pub badge_id: BadgeId,
  pub assigned_by: Option<PersonId>,
}
