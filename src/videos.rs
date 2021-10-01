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
pub struct Videos {
	future: Option<BoxFuture<'static, Result<Response, Error>>>,
	data: Option<VideosData>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct VideosData {
	key: ApiKey,
	part: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	id: Option<String>,
}

impl Videos {
	const URL: &'static str = "https://www.googleapis.com/youtube/v3/videos";

	/// create struct with an [`ApiKey`](../struct.ApiKey.html)
	#[must_use]
	pub fn new(key: ApiKey) -> Self {
		Self {
			future: None,
			data: Some(VideosData {
				key,
				part: String::from("snippet,contentDetails"),
				id: None
			}),
		}
	}

	#[must_use]
	pub fn id(mut self, id: &str) -> Self {
		let mut data = self.data.take().unwrap();
		data.id = Some(id.into());
		self.data = Some(data);
		self
	}

}

impl Future for Videos {
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
	pub items: Vec<VideoResult>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
	pub total_results: i64,
	pub results_per_page: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct VideoResult {
	pub kind: String,
	pub etag: String,
	pub id: String,
	pub snippet: Snippet,
    pub content_details: ContentDetails,
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
    pub category_id: Option<String>,
	pub live_broadcast_content: Option<String>,
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
pub struct ContentDetails {
	pub duration: Option<String>,
	pub dimension: Option<String>,	
    pub definition: Option<String>
}
