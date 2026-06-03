use serde::Deserialize;

use super::{LikeStatus, RepeatMode, Thumbnail, TrackState};

/// <https://ytmdesktop.github.io/developer/companion-server/reference/v1/auth-requestcode.html#response>
#[derive(Deserialize, Debug)]
pub struct AuthCodeResponse {
	pub code: String,
}

/// <https://ytmdesktop.github.io/developer/companion-server/reference/v1/auth-request.html#response>
#[derive(Deserialize, Debug)]
pub struct AuthResponse {
	pub token: String,
}

/// <https://ytmdesktop.github.io/developer/companion-server/reference/v1/state.html#response>
#[derive(Deserialize, Debug, Clone)]
pub struct StateResponse {
	pub player: Player,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub video: Option<Video>,
	#[serde(rename = "playlistId", skip_serializing_if = "Option::is_none")]
	pub playlist_id: Option<String>,
}

/// <https://ytmdesktop.github.io/developer/companion-server/reference/v1/state.html#player-object>
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Player {
	pub ad_playing: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub queue: Option<Queue>,
	pub track_state: TrackState,
	pub video_progress: u32,
	pub volume: u32,
}

/// <https://ytmdesktop.github.io/developer/companion-server/reference/v1/state.html#queue-object>
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Queue {
	pub autoplay: bool,
	pub items: Vec<QueueItem>,
	pub automix_items: Vec<QueueItem>,
	pub is_generating: bool,
	pub is_infinite: bool,
	pub repeat_mode: RepeatMode,
	pub selected_item_index: u32,
}

/// <https://ytmdesktop.github.io/developer/companion-server/reference/v1/state.html#queue-item-object>
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueueItem {
	pub thumbnails: Vec<Thumbnail>,
	pub title: String,
	pub author: String,
	pub duration: String,
	pub selected: bool,
	pub video_id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub counterparts: Option<Vec<QueueItem>>,
}

/// <https://ytmdesktop.github.io/developer/companion-server/reference/v1/state.html#video-object>
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Video {
	pub author: String,
	pub channel_id: String,
	pub title: String,
	pub album: String,
	pub album_id: String,
	pub like_status: LikeStatus,
	pub thumbnails: Vec<Thumbnail>,
	pub duration_seconds: u32,
	pub id: String,
}
