use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header, request::Parts},
    response::{IntoResponse, Redirect, Response},
};

#[derive(Debug, Clone)]
pub struct AdminAuth {
    pub username: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AdminAuth
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let cookie_header = parts
            .headers
            .get(header::COOKIE)
            .and_then(|h| h.to_str().ok())
            .unwrap_or("");

        let is_admin_logged_in = cookie_header
            .split(';')
            .any(|cookie| cookie.trim().starts_with("meincms_admin_session="));

        if is_admin_logged_in {
            let username = cookie_header
                .split(';')
                .find(|c| c.trim().starts_with("meincms_admin_session="))
                .and_then(|c| c.split('=').nth(1))
                .unwrap_or("admin")
                .to_string();

            Ok(AdminAuth { username })
        } else {
            let original_uri = parts.uri.path_and_query().map(|pq| pq.as_str()).unwrap_or("/");
            let login_url = format!("/login?redirect={}", original_uri);
            Err(Redirect::to(&login_url).into_response())
        }
    }
}
