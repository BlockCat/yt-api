use std::env;

use yt_api::{
	search::{Error, ItemType, SearchList},
	ApiKey,
};

/// prints the first answer of a search query
fn main() -> Result<(), Error> {
	futures::executor::block_on(async {
		// take api key from enviroment variable
		let key = ApiKey::new(&env::var("YT_API_KEY").expect("YT_API_KEY env-var not found"));

		// create the SearchList struct for the query "rust lang"
		let result = SearchList::new(key)
			.q("rust lang")
			.item_type(ItemType::Video)
			.await?;

		// outputs the title of the first search result
		println!(
			"Title: \"{}\"",
			result.items[0].snippet.title.as_ref().unwrap()
		);
		// outputs the video id of the first search result
		println!(
			"https://youtube.com/watch?v={}",
			result.items[0].id.video_id.as_ref().unwrap()
		);

		println!(
			"Default thumbnail: {}",
			result.items[0]
				.snippet
				.thumbnails
				.as_ref()
				.unwrap()
				.default
				.as_ref()
				.unwrap()
				.url
		);

		Ok(())
	})
}
