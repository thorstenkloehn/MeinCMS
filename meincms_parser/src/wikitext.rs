use regex::Regex;
use std::sync::OnceLock;

static CATEGORY_REGEX: OnceLock<Regex> = OnceLock::new();
static BOLD_REGEX: OnceLock<Regex> = OnceLock::new();
static ITALIC_REGEX: OnceLock<Regex> = OnceLock::new();
static LINK_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_category_regex() -> &'static Regex {
    CATEGORY_REGEX.get_or_init(|| Regex::new(r"(?i)\[\[(?:kategorie|category):(.*?)\]\]").unwrap())
}

fn get_bold_regex() -> &'static Regex {
    BOLD_REGEX.get_or_init(|| Regex::new(r"'''(.*?)'''").unwrap())
}

fn get_italic_regex() -> &'static Regex {
    ITALIC_REGEX.get_or_init(|| Regex::new(r"''(.*?)''").unwrap())
}

fn get_link_regex() -> &'static Regex {
    LINK_REGEX.get_or_init(|| Regex::new(r"\[\[(.*?)\]\]").unwrap())
}

pub fn to_html(wiki_text: &str) -> String {
    if wiki_text.trim().is_empty() {
        return String::new();
    }

    let processed_text = crate::scripting::process_rhai_macros(wiki_text);
    let mut result = String::new();
    let lines: Vec<&str> = processed_text.lines().collect();

    for line in lines {
        let trimmed = line.trim();

        if trimmed.starts_with('=') && trimmed.ends_with('=') {
            let level = trimmed.chars().take_while(|&c| c == '=').count();
            if level >= 1 && level <= 6 {
                let content = trimmed.trim_matches('=').trim();
                result.push_str(&format!(
                    "<h{level}>{}</h{level}>\n",
                    html_escape::encode_text(content)
                ));
                continue;
            }
        }

        if get_category_regex().is_match(trimmed) {
            continue;
        }

        if !trimmed.is_empty() {
            let processed = process_inline_wikitext(line);
            result.push_str(&format!("<p>{processed}</p>\n"));
        }
    }

    result
}

pub fn get_categories(wiki_text: &str) -> Vec<String> {
    let mut categories = Vec::new();
    for cap in get_category_regex().captures_iter(wiki_text) {
        if let Some(cat) = cap.get(1) {
            categories.push(cat.as_str().trim().to_string());
        }
    }
    categories
}

fn process_inline_wikitext(text: &str) -> String {
    let without_cats = get_category_regex().replace_all(text, "");
    let with_bold = get_bold_regex().replace_all(&without_cats, "<strong>$1</strong>");
    let with_italic = get_italic_regex().replace_all(&with_bold, "<em>$1</em>");

    let with_links = get_link_regex().replace_all(&with_italic, |caps: &regex::Captures| {
        let content = caps.get(1).unwrap().as_str();
        let parts: Vec<&str> = content.split('|').collect();
        if parts.len() > 1 {
            format!(
                "<a href=\"/wiki/{}\">{}</a>",
                html_escape::encode_double_quoted_attribute(parts[0].trim()),
                html_escape::encode_text(parts[1].trim())
            )
        } else {
            format!(
                "<a href=\"/wiki/{}\">{}</a>",
                html_escape::encode_double_quoted_attribute(content.trim()),
                html_escape::encode_text(content.trim())
            )
        }
    });

    with_links.to_string()
}
