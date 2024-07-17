use anyhow::anyhow;
use google_maps::{distance_matrix::response::element_status::ElementStatus, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq, Clone)]
pub struct DistanceQuery {
    pub from: String,
    pub dest: String,
}

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq, Clone)]
pub struct DistanceResponse {
    pub distance_meters: Option<u32>,
    pub distance_text: Option<String>,
    pub duration_secs: Option<i64>,
    pub duration_text: Option<String>,
}

pub async fn fetch_distance(
    DistanceQuery { from, dest }: &DistanceQuery,
    api_key: &str,
) -> anyhow::Result<DistanceResponse> {
    let google_maps_client = GoogleMapsClient::try_new(api_key).unwrap();

    let distance_matrix = google_maps_client
        .distance_matrix(
            // Origins
            vec![Waypoint::from_address(from)],
            // Destinations
            vec![Waypoint::from_address(dest)],
        )
        .execute()
        .await?;

    if !matches!(distance_matrix.status, DistanceMatrixStatus::Ok) {
        return Err(anyhow!(
            "an error occurred when fetching the distance between {} and {}: {}",
            &from,
            &dest,
            distance_matrix.status
        ));
    }

    let response = distance_matrix.rows[0].elements[0].clone();

    match response.status {
        ElementStatus::Ok => {}
        ElementStatus::MaxRouteLengthExceeded => {
            return Err(anyhow!(
                "an error occurred when fetching the distance between '{}' and '{}': {}",
                from,
                dest,
                response.status
            ))
        }
        ElementStatus::NotFound => {
            return Err(anyhow!(
                "an error occurred when fetching the distance between '{}' and '{}': {}",
                from,
                dest,
                response.status
            ))
        }
        ElementStatus::ZeroResults => {
            return Err(anyhow!(
                "an error occurred when fetching the distance between '{}' and '{}': {}",
                from,
                dest,
                response.status
            ))
        }
    }

    Ok(DistanceResponse {
        distance_meters: response.distance.as_ref().map(|d| d.value),
        distance_text: response.distance.map(|d| d.text),
        duration_secs: response.duration.as_ref().map(|d| d.value.num_seconds()),
        duration_text: response.duration.map(|d| d.text),
    })
}
