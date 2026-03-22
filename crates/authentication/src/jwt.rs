
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // User's unique ID
    iss: String, // Issuer (should match your Authentik URL)
    exp: usize,  // Expiration time
}

pub async fn validate_jwt(
    jwks_url: &str,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // 2. Fetch the keys (In production, CACHE this result to avoid network calls on every request!)
    let jwks: JwkSet = reqwest::get(jwks_url).await?.json().await?;

    // 3. Get the Key ID (kid) from the JWT header
    let header = decode_header(token)?;
    let kid = header.kid.ok_or("No 'kid' in token header")?;

    // 4. Find the matching key in the JWKS set
    let jwk = jwks.find(&kid).ok_or("No matching key found in JWKS")?;

    // 5. Create the decoding key from the JWK
    let decoding_key = DecodingKey::from_jwk(jwk)?;

    // 6. Validate the token
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&["<your-client-id>"]); // Must match the Client ID in Authentik

    let decoded_token = decode::<Claims>(token, &decoding_key, &validation)?;

    println!("Successfully validated user: {}", decoded_token.claims.sub);
    Ok(())
}