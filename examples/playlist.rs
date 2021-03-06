use std::env;

use yt_api::{
	playlistitems::{Error, PlaylistItems},
	ApiKey,
};

/// prints the first answer of a search query
fn main() -> Result<(), Error> {
	futures::executor::block_on(async {
		// take api key from enviroment variable
		let key = ApiKey::new(&env::var("YT_API_KEY").expect("YT_API_KEY env-var not found"));

		// create the PlaylistItems struct for some playlist ID
		let result = PlaylistItems::new(key)
			.playlist_id("PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL")
			.max_results(50)
			.await?;

		for item in result.items {
			println!(
				"https://youtube.com/watch?v={}",
				item.snippet.resource_id.video_id
			);
		}

		Ok(())
	})
}
