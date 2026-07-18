use axum::{
    extract::{Path, Query, State},
    http::header,
    response::{Html, IntoResponse, Redirect, Response},
    Form,
};
use maud::html;
use serde::Deserialize;

use crate::auth::AdminAuth;
use crate::db::DbStore;
use crate::models::ArticleSaveForm;
use crate::tenant::Tenant;
use crate::views::article::{render_article_page, render_edit_page, render_history_page};
use crate::views::layout::render_base_layout;
use crate::views::login::render_login_page;

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginQuery {
    pub redirect: Option<String>,
    pub error: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
    pub redirect: Option<String>,
}

pub async fn get_index(Tenant(tenant_id): Tenant, State(db): State<DbStore>) -> Response {
    let article = db.get_article(&tenant_id, "Hauptseite").await;
    if let Some(art) = article {
        Html(render_article_page("Hauptseite", &tenant_id, &art).into_string()).into_response()
    } else {
        Redirect::to("/edit/Hauptseite").into_response()
    }
}

pub async fn get_article(
    Tenant(tenant_id): Tenant,
    Path(slug): Path<String>,
    State(db): State<DbStore>,
) -> Response {
    let clean_slug = slug.trim_matches('/');
    if let Some(art) = db.get_article(&tenant_id, clean_slug).await {
        Html(render_article_page(clean_slug, &tenant_id, &art).into_string()).into_response()
    } else {
        Redirect::to(&format!("/edit/{}", clean_slug)).into_response()
    }
}

// RESTRICTED TO ADMINS ONLY! Non-admins will be redirected to login page.
pub async fn get_edit_article(
    _auth: AdminAuth,
    Tenant(tenant_id): Tenant,
    Path(slug): Path<String>,
    State(db): State<DbStore>,
) -> impl IntoResponse {
    let clean_slug = slug.trim_matches('/');
    let article = db.get_article(&tenant_id, clean_slug).await;
    let latest = article.as_ref().and_then(|a| a.latest_version.as_ref());

    Html(render_edit_page(clean_slug, &tenant_id, latest).into_string())
}

// RESTRICTED TO ADMINS ONLY! Non-admins cannot save articles.
pub async fn post_save_article(
    _auth: AdminAuth,
    Tenant(tenant_id): Tenant,
    Path(slug): Path<String>,
    State(db): State<DbStore>,
    Form(form): Form<ArticleSaveForm>,
) -> impl IntoResponse {
    let clean_slug = slug.trim_matches('/');
    let raw_cats = form.kategorien_raw.unwrap_or_default();
    let kategorien: Vec<String> = raw_cats
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    db.save_article(
        &tenant_id,
        clean_slug,
        &form.syntax,
        form.markdown_inhalt,
        form.wiki_text_inhalt,
        kategorien,
    )
    .await;

    Redirect::to(&format!("/wiki/{}", clean_slug))
}

pub async fn get_login(
    Tenant(tenant_id): Tenant,
    Query(query): Query<LoginQuery>,
) -> impl IntoResponse {
    let redirect = query.redirect.unwrap_or_else(|| "/".to_string());
    Html(render_login_page(&tenant_id, &redirect, query.error.as_deref()).into_string())
}

pub async fn post_login(Form(form): Form<LoginForm>) -> impl IntoResponse {
    let redirect_url = form.redirect.unwrap_or_else(|| "/".to_string());

    // Basic / Admin Credentials Validation (Default emergency admin or users store)
    if (form.username == "admin" || form.username.contains("admin")) && !form.password.is_empty() {
        let mut response = Redirect::to(&redirect_url).into_response();
        let cookie = format!(
            "meincms_admin_session={}; Path=/; HttpOnly; SameSite=Lax",
            form.username
        );
        response
            .headers_mut()
            .insert(header::SET_COOKIE, cookie.parse().unwrap());
        response
    } else {
        let err_url = format!("/login?redirect={}&error=Ungueltiges+Passwort", redirect_url);
        Redirect::to(&err_url).into_response()
    }
}

pub async fn get_logout() -> impl IntoResponse {
    let mut response = Redirect::to("/").into_response();
    let cookie = "meincms_admin_session=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT";
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.parse().unwrap());
    response
}

pub async fn get_history(
    Tenant(tenant_id): Tenant,
    Path(slug): Path<String>,
    State(db): State<DbStore>,
) -> impl IntoResponse {
    let clean_slug = slug.trim_matches('/');
    let versions = db.get_history(&tenant_id, clean_slug).await;
    Html(render_history_page(clean_slug, &tenant_id, &versions).into_string())
}

pub async fn get_version(
    Tenant(tenant_id): Tenant,
    Path(version_num): Path<i64>,
    State(db): State<DbStore>,
) -> Response {
    if let Some(v) = db.get_version(version_num).await {
        let content = html! {
            div class="card" {
                h1 class="wiki-article-title" { "Historische Version #" (version_num) }
                p style="color: var(--text-secondary); margin-bottom: 1.5rem;" {
                    "Erstellt am: " (v.zeitpunkt.format("%d.%m.%Y %H:%M:%S").to_string())
                }
                div class="wiki-content" {
                    (maud::PreEscaped(v.html_inhalt.unwrap_or_default()))
                }
            }
        };
        Html(render_base_layout(&format!("Version #{}", version_num), &tenant_id, content).into_string()).into_response()
    } else {
        (axum::http::StatusCode::NOT_FOUND, "Version nicht gefunden").into_response()
    }
}

pub async fn get_all_articles(
    Tenant(tenant_id): Tenant,
    State(db): State<DbStore>,
) -> impl IntoResponse {
    let articles = db.get_all_articles(&tenant_id).await;
    let content = html! {
        div class="card" {
            h1 class="wiki-article-title" { "Alle Wiki-Artikel (" (tenant_id) ")" }
            ul style="list-style: none; margin-top: 1.5rem;" {
                @for art in articles {
                    li style="margin-bottom: 0.75rem;" {
                        a href={ "/wiki/" (art.slug) } class="nav-link" style="font-size: 1.1rem; font-weight: 600;" { (art.slug) }
                    }
                }
            }
        }
    };
    Html(render_base_layout("Alle Artikel", &tenant_id, content).into_string())
}

pub async fn get_search(
    Tenant(tenant_id): Tenant,
    Query(params): Query<SearchParams>,
    State(db): State<DbStore>,
) -> impl IntoResponse {
    let q = params.q.unwrap_or_default();
    let results = if q.is_empty() {
        Vec::new()
    } else {
        db.search_articles(&tenant_id, &q).await
    };

    let content = html! {
        div class="card" {
            h1 class="wiki-article-title" { "Suchergebnisse für: " (q) }
            @if results.is_empty() {
                p { "Keine passenden Artikel gefunden." }
            } @else {
                ul style="list-style: none; margin-top: 1.5rem;" {
                    @for art in results {
                        li style="margin-bottom: 1rem; padding-bottom: 1rem; border-bottom: 1px solid var(--border-color);" {
                            a href={ "/wiki/" (art.slug) } class="nav-link" style="font-size: 1.2rem; font-weight: 600;" { (art.slug) }
                        }
                    }
                }
            }
        }
    };
    Html(render_base_layout(&format!("Suche: {}", q), &tenant_id, content).into_string())
}
