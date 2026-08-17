use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts, StatusCode},
};

#[derive(Debug, Clone)]
pub struct Tenant(pub String);

impl<S> FromRequestParts<S> for Tenant
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let host = parts
            .headers
            .get(header::HOST)
            .and_then(|h| h.to_str().ok())
            .unwrap_or("localhost")
            .split(':')
            .next()
            .unwrap_or("localhost")
            .to_lowercase();

        let tenant_id = match host.as_str() {
            "doc" | "doc.localhost" | "doc.wissen-ahrensburg.de" => "doc".to_string(),
            _ => "main".to_string(),
        };

        Ok(Tenant(tenant_id))
    }
}
