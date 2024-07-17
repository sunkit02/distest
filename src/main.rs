use anyhow::Result;
use cli::parse_cli_args;
use data::DistanceQuery;
use tokio;

mod caching;
mod cli;
mod configs;
mod data;

#[tokio::main]
async fn main() -> Result<()> {
    let args = parse_cli_args()?;

    let path = caching::get_default_cache_path();
    let mut query_cache = caching::get_cache(&path).await;

    let query = DistanceQuery {
        from: args.from,
        dest: args.dest,
    };

    if !query_cache.contains_key(&query) {
        eprintln!("Query not in cache, fetching from internet...");
        eprintln!();
        let response = data::fetch_distance(&query, &args.api_key).await?;
        query_cache.insert(query.clone(), response);
    }

    let response = query_cache
        .get(&query)
        .expect("response entry should exist in cache");

    println!(
        "Distance:    {}",
        response.distance_text.as_ref().unwrap_or(&"N/A".to_owned())
    );
    println!(
        "Travel time: {}",
        response.duration_text.as_ref().unwrap_or(&"N/A".to_owned())
    );

    caching::flush_cache(&path, &query_cache).await;

    Ok(())
}
