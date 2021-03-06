# yt-api

[![Crates.io](https://img.shields.io/crates/v/yt-api.svg)](https://crates.io/crates/yt-api)
[![Documentation](https://docs.rs/yt-api/badge.svg)](https://docs.rs/yt-api)

## about
With yt-api you can interact asynchronously with the youtube-api.
Currently it implements the following endpoints:
 * search
 * playlists

## examples
A basic search request with yt-api:

``` rust
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

        Ok(())
    })
}
```

A basic playlist request with yt-api:

``` rust
/// prints the first answer of a search query
fn main(video_id: String) -> Result<(), Error> {
    futures::executor::block_on(async {
        // take api key from enviroment variable
        let yt_token = env::var("YT_KEY")
        .expect("Expected youtube token in environment");

        // create the PlaylistItems struct for some playlist ID
        let result = PlaylistItems::new(ApiKey::new(yt_token))
        .playlist_id(video_id)
        .max_results(50)
        .await?;

        let mut arg: String;
        for item in result.items {
            println!(
                "https://youtube.com/watch?v={}",
                item.snippet.resource_id.video_id
            );
        }

        Ok(())
    })
}
```

More examples can be found [here](examples). 

## supported rust versions

the minimum rust version for yt-api is 1.39

## license

This project is licensed under the [MIT license](LICENSE).

## contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in yt-api by you, shall be licensed as MIT, without any additional
terms or conditions.

