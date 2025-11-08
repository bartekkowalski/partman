use reqwest::{self, header};
use serde::Deserialize;

use crate::{api::digikey::product::ApiResponse, Result};
mod product;

pub use product::DigikeyProduct;

/// Response structure for OAuth token requests
#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
}

pub fn get_product(client_id: &str, client_secret: &str, dk_mpn: &str) -> Result<DigikeyProduct> {
    let client = reqwest::blocking::Client::new();

    // Get access token
    let params = [
        ("client_id", &client_id),
        ("client_secret", &client_secret),
        ("grant_type", &"client_credentials"),
    ];
    let response = client
        .post("https://api.digikey.com/v1/oauth2/token")
        .form(&params)
        .send()?;
    let token_response: TokenResponse = response.json()?;
    let token = token_response.access_token;


    // Get Digikey product information
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {token}"))?,
    );
    headers.insert(
        "X-DIGIKEY-Client-Id",
        header::HeaderValue::from_str(client_id)?,
    );
    headers.insert(
        "X-DIGIKEY-Locale-Site",
        header::HeaderValue::from_static("AU"),
    );
    headers.insert(
        "X-DIGIKEY-Locale-Language",
        header::HeaderValue::from_static("en"),
    );
    headers.insert(
        "X-DIGIKEY-Locale-Currency",
        header::HeaderValue::from_static("AUD"),
    );
    let response = client
        .get(format!(
            "https://api.digikey.com/products/v4/search/{}/productdetails",
            &dk_mpn
        ))
        .headers(headers)
        .send()?;

    let mut api_response: ApiResponse = response.json()?;

    // Store the search SPN since DigiKey may return multiple part numbers
    api_response.product.search_spn = Some(dk_mpn.to_owned());

    Ok(api_response.product)
}