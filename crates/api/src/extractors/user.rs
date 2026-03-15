use super::errors::ExtractorError;
use crate::models::User;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::HeaderMap;
use axum::http::header;
use axum::http::request::Parts;
use tracing::debug;
use uuid::Uuid;


// #[async_trait]
// impl<S> FromRequestParts<S> for Option<User>
// where
//     S: Send + Sync,
// {
//     type Rejection = ExtractorError;

//     async fn from_request_parts(
//         parts: &mut Parts,
//         state: &S,
//     ) -> Result<Self, Self::Rejection> {
//         let headers = HeaderMap::from_request_parts(parts, state)
//             .await
//             .map_err(anyhow::Error::from)?;
//         let header = match headers.get(header::AUTHORIZATION) {
//             Some(header) => header,
//             None => {
//                 debug!("Anonymous user");
//                 return Ok(None);
//             }
//         };

//         let mut token: String = header.to_str().unwrap().to_string();
//         let token = token
//             .strip_prefix("Bearer ")
//             .unwrap_or(token.as_str())
//             .to_string();
//         let user = User::new(&Uuid::now_v7(), &"name");
//         Ok(Some(user))
//     }
// }

// // impl<S> FromRequestParts<S> for User
// // where
// //     S: Send + Sync,
// // {
// //     type Rejection = ExtractorError;

// //     async fn from_request_parts(
// //         parts: &mut Parts,
// //         state: &S,
// //     ) -> Result<Self, Self::Rejection> {
// //         match Option<User>::from_request_parts(parts, state).await{
// //             Ok(user) => Ok(user),
// //             None => Err(ExtractorError::NotLoggedIn)
// //         }
// //     }
// // }
