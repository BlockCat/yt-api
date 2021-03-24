use std::{
	future::Future,
	pin::Pin,
	task::{Context, Poll},
};

use chrono::{DateTime, Utc};
use futures::future::BoxFuture;
use log::debug;
use serde::{Deserialize, Serialize, Serializer};
use snafu::{ResultExt, Snafu};

use super::ApiKey;

/// custom error type for the search endpoint
#[derive(Debug, Snafu)]
pub enum Error {
	#[snafu(display("failed to connect to the api: {}", string))]
	Connection { string: String },
	#[snafu(display("failed to deserialize: {} {}", string, source))]
	Deserialization {
		string: String,
		source: serde_json::Error,
	},
	#[snafu(display("failed to serialize: {}", source))]
	Serialization {
		source: serde_urlencoded::ser::Error,
	},
}

impl From<surf::Error> for Error {
	fn from(surf_error: surf::Error) -> Self {
		Error::Connection {
			string: surf_error.to_string(),
		}
	}
}

/// request struct for the search endpoint
pub struct PlaylistItems {
	future: Option<BoxFuture<'static, Result<Response, Error>>>,
	data: Option<PlaylistItemsData>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct PlaylistItemsData {
	key: ApiKey,
	part: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	max_results: Option<u8>,
	#[serde(skip_serializing_if = "Option::is_none")]
	on_behalf_of_content_owner: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	page_token: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	playlist_id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	video_id: Option<String>,
}

impl PlaylistItems {
	const URL: &'static str = "https://www.googleapis.com/youtube/v3/playlistItems";

	/// create struct with an [`ApiKey`](../struct.ApiKey.html)
	#[must_use]
	pub fn new(key: ApiKey) -> Self {
		Self {
			future: None,
			data: Some(PlaylistItemsData {
				key,
				part: String::from("snippet"),
				id: None,
				max_results: None,
				on_behalf_of_content_owner: None,
				page_token: None,
				playlist_id: None,
				video_id: None,
			}),
		}
	}

	#[must_use]
	pub fn id(mut self, id: impl Into<String>) -> Self {
		let mut data = self.data.take().unwrap();
		data.id = Some(id.into());
		self.data = Some(data);
		self
	}

	#[must_use]
	pub fn max_results(mut self, max_results: impl Into<u8>) -> Self {
		let mut data = self.data.take().unwrap();
		data.max_results = Some(max_results.into());
		self.data = Some(data);
		self
	}

	#[must_use]
	pub fn on_behalf_of_content_owner(
		mut self,
		on_behalf_of_content_owner: impl Into<String>,
	) -> Self {
		let mut data = self.data.take().unwrap();
		data.on_behalf_of_content_owner = Some(on_behalf_of_content_owner.into());
		self.data = Some(data);
		self
	}

	#[must_use]
	pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
		let mut data = self.data.take().unwrap();
		data.page_token = Some(page_token.into());
		self.data = Some(data);
		self
	}

	#[must_use]
	pub fn playlist_id(mut self, playlist_id: impl Into<String>) -> Self {
		let mut data = self.data.take().unwrap();
		data.playlist_id = Some(playlist_id.into());
		self.data = Some(data);
		self
	}

	#[must_use]
	pub fn video_id(mut self, video_id: impl Into<String>) -> Self {
		let mut data = self.data.take().unwrap();
		data.video_id = Some(video_id.into());
		self.data = Some(data);
		self
	}
}

impl Future for PlaylistItems {
	type Output = Result<Response, Error>;

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		if self.future.is_none() {
			let data = self.data.take().unwrap();
			self.future = Some(Box::pin(async move {
				let url = format!(
					"{}?{}",
					Self::URL,
					serde_urlencoded::to_string(&data).context(Serialization)?
				);
				debug!("getting {}", url);
				let response = surf::get(&url).recv_string().await?;
				serde_json::from_str(&response)
					.with_context(move || Deserialization { string: response })
			}));
		}

		self.future.as_mut().unwrap().as_mut().poll(cx)
	}
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChannelType {
	Any,
	Show,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum EventType {
	Completed,
	Live,
	Upcoming,
}

#[derive(Debug, Clone)]
pub struct VideoLocation {
	longitude: f32,
	latitude: f32,
}

impl VideoLocation {
	#[must_use]
	pub fn new(longitude: f32, latitude: f32) -> Self {
		Self {
			longitude,
			latitude,
		}
	}
}

impl Serialize for VideoLocation {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&format!("{},{}", self.longitude, self.latitude))
	}
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Order {
	Date,
	Rating,
	Relevance,
	Title,
	VideoCount,
	ViewCount,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SafeSearch {
	Moderate,
	Strict,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ItemType {
	Channel,
	Playlist,
	Video,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VideoCaption {
	ClosedCaption,
	None,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VideoDefinition {
	High,
	Standard,
}

#[derive(Debug, Clone, Serialize)]
pub enum VideoDimension {
	#[serde(rename = "3d")]
	Three,
	#[serde(rename = "2d")]
	Two,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VideoDuration {
	Long,
	Medium,
	Short,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VideoLicense {
	CreativeCommon,
	Youtube,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VideoType {
	Episode,
	Movie,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
	pub kind: String,
	pub etag: String,
	pub next_page_token: Option<String>,
	pub prev_page_token: Option<String>,
	pub page_info: PageInfo,
	pub items: Vec<PlaylistResult>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
	pub total_results: i64,
	pub results_per_page: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlaylistResult {
	pub kind: String,
	pub etag: String,
	pub id: String,
	pub snippet: Snippet,
	pub content_details: Option<ContentDetails>,
	pub status: Option<Status>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
	pub published_at: Option<DateTime<Utc>>,
	pub channel_id: Option<String>,
	pub title: Option<String>,
	pub description: Option<String>,
	pub thumbnails: Option<Thumbnails>,
	pub channel_title: Option<String>,
	pub video_owner_channel_title: Option<String>,
	pub video_owner_channel_id: Option<String>,
	pub playlist_id: Option<String>,
	pub position: Option<u32>,
	pub resource_id: Resource,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Thumbnails {
	pub default: Option<Thumbnail>,
	pub medium: Option<Thumbnail>,
	pub high: Option<Thumbnail>,
	pub standard: Option<Thumbnail>,
	pub maxres: Option<Thumbnail>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Thumbnail {
	pub url: String,
	pub width: Option<u64>,
	pub height: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
	pub kind: String,
	pub video_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentDetails {
	pub video_id: String,
	pub start_at: String,
	pub end_at: String,
	pub note: String,
	pub video_published_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Status {
	pub privacy_status: String,
}
