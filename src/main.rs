use anyhow::{anyhow, Result};
use cli::parse_cli_args;
use google_maps::{distance_matrix::response::element_status::ElementStatus, prelude::*};
use tokio;

mod cli;
mod configs;

#[tokio::main]
async fn main() -> Result<()> {
    let args = parse_cli_args()?;

    let google_maps_client = GoogleMapsClient::try_new(args.api_key).unwrap();

    // Example request:

    let distance_matrix = google_maps_client
        .distance_matrix(
            // Origins
            vec![Waypoint::from_address(&args.from)],
            // Destinations
            vec![Waypoint::from_address(&args.destination)],
        )
        .execute()
        .await?;

    // Dump entire response:
    if !matches!(distance_matrix.status, DistanceMatrixStatus::Ok) {
        return Err(anyhow!(
            "an error occurred when fetching the distance between {} and {}: {}",
            args.from,
            args.destination,
            distance_matrix.status
        ));
    }

    let response = &distance_matrix.rows[0].elements[0];

    match response.status {
        ElementStatus::Ok => {}
        ElementStatus::MaxRouteLengthExceeded => {
            return Err(anyhow!(
                "an error occurred when fetching the distance between '{}' and '{}': {}",
                args.from,
                args.destination,
                response.status
            ))
        }
        ElementStatus::NotFound => {
            return Err(anyhow!(
                "an error occurred when fetching the distance between '{}' and '{}': {}",
                args.from,
                args.destination,
                response.status
            ))
        }
        ElementStatus::ZeroResults => {
            return Err(anyhow!(
                "an error occurred when fetching the distance between '{}' and '{}': {}",
                args.from,
                args.destination,
                response.status
            ))
        }
    }

    println!(
        "Distance:    {}",
        response
            .distance
            .as_ref()
            .map(|d| d.text.as_ref())
            .unwrap_or("N/A")
    );
    println!(
        "Travel time: {}",
        response
            .duration
            .as_ref()
            .map(|d| d.text.as_ref())
            .unwrap_or("N/A")
    );

    Ok(())
}
