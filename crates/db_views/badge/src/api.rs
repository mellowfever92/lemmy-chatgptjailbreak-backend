use lemmy_db_schema::newtypes::{BadgeId, PersonId};
use lemmy_diesel_utils::dburl::DbUrl;
use serde::{Deserialize, Serialize};

use crate::BadgeView;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(optional_fields, export))]
/// Create a badge.
pub struct CreateBadge {
  pub name: String,
  pub description: Option<String>,
  pub image_url: DbUrl,
  pub is_assignable_by_mods: bool,
  pub is_self_selectable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(optional_fields, export))]
/// A response for a badge.
pub struct BadgeResponse {
  pub badge: BadgeView,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(optional_fields, export))]
/// Delete a badge.
pub struct DeleteBadge {
  pub id: BadgeId,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(optional_fields, export))]
/// Edit a badge.
pub struct EditBadge {
  pub id: BadgeId,
  pub name: Option<String>,
  pub description: Option<Option<String>>,
  pub image_url: Option<DbUrl>,
  pub is_assignable_by_mods: Option<bool>,
  pub is_self_selectable: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(optional_fields, export))]
/// List all badges.
pub struct ListBadges {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(optional_fields, export))]
/// A response containing a list of badges.
pub struct ListBadgesResponse {
  pub badges: Vec<BadgeView>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(optional_fields, export))]
/// Assign a badge to a person.
pub struct AssignBadge {
  pub person_id: PersonId,
  pub badge_id: BadgeId,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(optional_fields, export))]
/// Remove a badge from a person.
pub struct RemoveBadge {
  pub person_id: PersonId,
  pub badge_id: BadgeId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(optional_fields, export))]
/// A success response.
pub struct BadgeActionResponse {
  pub success: bool,
}
