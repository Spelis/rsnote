use chrono::Utc;
use humantime::format_duration;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tabled::Tabled;

use crate::helpers::truncate;

#[derive(Serialize, Deserialize, Debug, Clone, Tabled)]
#[tabled(display(String, "display_short"))]
pub struct Note {
	pub id: String,
	pub title: String,
	pub body: String,
	#[tabled(display("display_ts"))]
	pub edited: i64,
	#[tabled(display("display_ts"))]
	pub created: i64,
	#[tabled(display("display_tags"))]
	pub tags: Vec<String>,
}

fn display_tags(tags: &Vec<String>) -> String {
	tags.join(", ")
}
fn display_short(s: &String) -> String {
	truncate(s.to_string(), 20)
}
pub fn display_ts(ts: &i64) -> String {
	let now = Utc::now().timestamp();
	let diff = now.saturating_sub(*ts);

	if *ts == 0 {
		return "Never".to_string();
	}
	if *ts <= now {
		// Past time
		let duration = Duration::from_secs(diff as u64);
		format!("{} ago", format_duration(duration))
	} else {
		// Future time
		let duration = Duration::from_secs((ts - now) as u64);
		format!("in {}", format_duration(duration))
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Notes(pub Vec<Note>);
