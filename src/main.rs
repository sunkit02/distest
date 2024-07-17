use anyhow::Result;
use cli::parse_cli_args;
use tokio;

mod cli;
mod configs;
mod data;

#[tokio::main]
async fn main() -> Result<()> {
    let args = parse_cli_args()?;

    let response = data::fetch_distance(&args.from, &args.dest, &args.api_key).await?;

    println!(
        "Distance:    {}",
        response.distance_text.unwrap_or("N/A".to_owned())
    );
    println!(
        "Travel time: {}",
        response.duration_text.unwrap_or("N/A".to_owned())
    );

    Ok(())
}
