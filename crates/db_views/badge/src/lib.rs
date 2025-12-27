use lemmy_db_schema::source::badge::Badge;
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
}
