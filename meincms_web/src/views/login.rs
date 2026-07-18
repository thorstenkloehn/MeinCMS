use maud::{html, Markup};

use crate::views::layout::render_base_layout;

pub fn render_login_page(tenant_id: &str, redirect_url: &str, error_msg: Option<&str>) -> Markup {
    let content = html! {
        div class="card" style="max-width: 450px; margin: 3rem auto;" {
            h1 class="wiki-article-title" style="text-align: center; margin-bottom: 1rem;" { "🔒 Admin Login" }
            p style="text-align: center; color: var(--text-secondary); margin-bottom: 2rem;" {
                "Nur Administratoren haben Berechtigung, Wiki-Artikel zu erstellen oder zu bearbeiten."
            }

            @if let Some(err) = error_msg {
                div style="background: rgba(239, 68, 68, 0.2); border: 1px solid #ef4444; color: #fca5a5; padding: 0.75rem 1rem; border-radius: 8px; margin-bottom: 1.5rem; font-size: 0.9rem;" {
                    (err)
                }
            }

            form action="/login" method="post" style="display: flex; flex-direction: column; gap: 1.25rem;" {
                input type="hidden" name="redirect" value=(redirect_url);

                div class="form-group" {
                    label style="display: block; font-weight: 600; margin-bottom: 0.5rem;" { "Benutzername (E-Mail):" }
                    input type="text" name="username" required="" placeholder="admin@wissen-ahrensburg.de" style="width: 100%; background: var(--bg-primary); color: white; border: 1px solid var(--border-color); padding: 0.75rem 1rem; border-radius: 8px;" ;
                }

                div class="form-group" {
                    label style="display: block; font-weight: 600; margin-bottom: 0.5rem;" { "Passwort:" }
                    input type="password" name="password" required="" placeholder="••••••••••••" style="width: 100%; background: var(--bg-primary); color: white; border: 1px solid var(--border-color); padding: 0.75rem 1rem; border-radius: 8px;" ;
                }

                button type="submit" class="btn" style="width: 100%; justify-content: center; margin-top: 0.5rem;" {
                    "Anmelden"
                }
            }
        }
    };

    render_base_layout("Admin Login", tenant_id, content)
}
