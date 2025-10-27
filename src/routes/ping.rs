use axum::response::IntoResponse;

/**
 * Healthcheck endpoint
 */
pub async fn ping() -> impl IntoResponse {
  "Pong"
}