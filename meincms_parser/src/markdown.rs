use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarkdownTokenType {
    Heading,
    Bold,
    Italic,
    Link,
    Category,
    CodeInline,
    CodeBlock,
    List,
    Template,
    TableRow,
    TableDivider,
    Text,
    Newline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownToken {
    pub token_type: MarkdownTokenType,
    pub level: usize,
    pub value: String,
    pub parameters: Vec<String>,
}

impl MarkdownToken {
    pub fn new(token_type: MarkdownTokenType, value: impl Into<String>) -> Self {
        Self {
            token_type,
            level: 0,
            value: value.into(),
            parameters: Vec::new(),
        }
    }
}

static HEADING_REGEX: OnceLock<Regex> = OnceLock::new();
static LIST_REGEX: OnceLock<Regex> = OnceLock::new();
static TEMPLATE_REGEX: OnceLock<Regex> = OnceLock::new();
static TABLE_DIVIDER_REGEX: OnceLock<Regex> = OnceLock::new();
static CODE_BLOCK_START_REGEX: OnceLock<Regex> = OnceLock::new();
static CATEGORY_REGEX: OnceLock<Regex> = OnceLock::new();
static CODE_INLINE_REGEX: OnceLock<Regex> = OnceLock::new();
static BOLD_STAR_REGEX: OnceLock<Regex> = OnceLock::new();
static BOLD_UNDERSCORE_REGEX: OnceLock<Regex> = OnceLock::new();
static ITALIC_STAR_REGEX: OnceLock<Regex> = OnceLock::new();
static ITALIC_UNDERSCORE_REGEX: OnceLock<Regex> = OnceLock::new();
static LINK_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_heading_regex() -> &'static Regex {
    HEADING_REGEX.get_or_init(|| Regex::new(r"^(#{1,6})\s+(.*)$").unwrap())
}

fn get_list_regex() -> &'static Regex {
    LIST_REGEX.get_or_init(|| Regex::new(r"^(\s*)([*+-]|\d+\.)\s+(.*)$").unwrap())
}

fn get_template_regex() -> &'static Regex {
    TEMPLATE_REGEX.get_or_init(|| Regex::new(r"\{\{(.*?)\}\}").unwrap())
}

fn get_table_divider_regex() -> &'static Regex {
    TABLE_DIVIDER_REGEX.get_or_init(|| Regex::new(r"^\|[\s\-\|:]+\|$").unwrap())
}

fn get_code_block_start_regex() -> &'static Regex {
    CODE_BLOCK_START_REGEX.get_or_init(|| Regex::new(r"^(\s*)(```|''')\s*(.*)$").unwrap())
}

fn get_category_regex() -> &'static Regex {
    CATEGORY_REGEX.get_or_init(|| Regex::new(r"\[\[[kK]ategorie:(.*?)\]\]").unwrap())
}

fn get_code_inline_regex() -> &'static Regex {
    CODE_INLINE_REGEX.get_or_init(|| Regex::new(r"`(.*?)`").unwrap())
}

fn get_bold_star_regex() -> &'static Regex {
    BOLD_STAR_REGEX.get_or_init(|| Regex::new(r"\*\*(.*?)\*\*").unwrap())
}

fn get_bold_underscore_regex() -> &'static Regex {
    BOLD_UNDERSCORE_REGEX.get_or_init(|| Regex::new(r"__(.*?)__").unwrap())
}

fn get_italic_star_regex() -> &'static Regex {
    ITALIC_STAR_REGEX.get_or_init(|| Regex::new(r"\*(.*?)\*").unwrap())
}

fn get_italic_underscore_regex() -> &'static Regex {
    ITALIC_UNDERSCORE_REGEX.get_or_init(|| Regex::new(r"_(.*?)_").unwrap())
}

fn get_link_regex() -> &'static Regex {
    LINK_REGEX.get_or_init(|| Regex::new(r"\[(.*?)\]\((.*?)\)").unwrap())
}

pub fn tokenize(input: &str) -> Vec<MarkdownToken> {
    if input.is_empty() {
        return Vec::new();
    }

    let mut tokens = Vec::new();
    let lines: Vec<&str> = input
        .split_terminator('\n')
        .map(|s| s.trim_end_matches('\r'))
        .collect();
    let mut in_code_block = false;
    let mut current_code_marker = String::new();
    let mut current_code_content = String::new();

    for line in lines {
        let trimmed_line = line.trim();

        if in_code_block {
            if trimmed_line.ends_with(&current_code_marker)
                && trimmed_line.len() == current_code_marker.len()
            {
                tokens.push(MarkdownToken {
                    token_type: MarkdownTokenType::CodeBlock,
                    level: 0,
                    value: current_code_content.trim_end().to_string(),
                    parameters: Vec::new(),
                });
                in_code_block = false;
                current_code_content.clear();
                continue;
            }
            current_code_content.push_str(line);
            current_code_content.push('\n');
            continue;
        }

        if let Some(code_match) = get_code_block_start_regex().captures(line) {
            in_code_block = true;
            current_code_marker = code_match.get(2).unwrap().as_str().to_string();
            continue;
        }

        if line.trim().is_empty() {
            tokens.push(MarkdownToken::new(MarkdownTokenType::Newline, ""));
            continue;
        }

        if let Some(heading_match) = get_heading_regex().captures(line) {
            let hashes = heading_match.get(1).unwrap().as_str();
            let val = heading_match.get(2).unwrap().as_str();
            tokens.push(MarkdownToken {
                token_type: MarkdownTokenType::Heading,
                level: hashes.len(),
                value: val.to_string(),
                parameters: Vec::new(),
            });
            continue;
        }

        if trimmed_line.starts_with('|') && trimmed_line.ends_with('|') {
            if get_table_divider_regex().is_match(trimmed_line) && trimmed_line.contains('-') {
                tokens.push(MarkdownToken::new(MarkdownTokenType::TableDivider, ""));
            } else {
                tokens.push(MarkdownToken::new(
                    MarkdownTokenType::TableRow,
                    trimmed_line,
                ));
            }
            continue;
        }

        if let Some(list_match) = get_list_regex().captures(line) {
            let indent = list_match.get(1).unwrap().as_str();
            let val = list_match.get(3).unwrap().as_str();
            tokens.push(MarkdownToken {
                token_type: MarkdownTokenType::List,
                level: indent.len() / 2,
                value: val.to_string(),
                parameters: Vec::new(),
            });
            continue;
        }

        if let Some(template_match) = get_template_regex().captures(line) {
            let full_val = template_match.get(1).unwrap().as_str();
            let parts: Vec<&str> = full_val.split('|').collect();
            tokens.push(MarkdownToken {
                token_type: MarkdownTokenType::Template,
                level: 0,
                value: parts[0].trim().to_string(),
                parameters: parts.iter().skip(1).map(|p| p.trim().to_string()).collect(),
            });
            continue;
        }

        tokenize_inline(line, &mut tokens);
        tokens.push(MarkdownToken::new(MarkdownTokenType::Newline, ""));
    }

    tokens
}

fn tokenize_inline(text: &str, tokens: &mut Vec<MarkdownToken>) {
    if text.is_empty() {
        return;
    }

    let mut earliest_match: Option<(usize, usize, MarkdownTokenType, Vec<String>)> = None;

    fn check_regex(
        text: &str,
        regex: &Regex,
        token_type: MarkdownTokenType,
        extractor: impl Fn(&regex::Captures) -> (String, Vec<String>),
        earliest: &mut Option<(usize, usize, MarkdownTokenType, Vec<String>)>,
    ) {
        if let Some(mat) = regex.captures(text) {
            let m_match = mat.get(0).unwrap();
            let start = m_match.start();
            let end = m_match.end();
            let (val, params) = extractor(&mat);

            if earliest.is_none() || start < earliest.as_ref().unwrap().0 {
                let mut p = vec![val];
                p.extend(params);
                *earliest = Some((start, end, token_type, p));
            }
        }
    }

    check_regex(
        text,
        get_category_regex(),
        MarkdownTokenType::Category,
        |c| (c.get(1).unwrap().as_str().trim().to_string(), vec![]),
        &mut earliest_match,
    );
    check_regex(
        text,
        get_code_inline_regex(),
        MarkdownTokenType::CodeInline,
        |c| (c.get(1).unwrap().as_str().to_string(), vec![]),
        &mut earliest_match,
    );
    check_regex(
        text,
        get_template_regex(),
        MarkdownTokenType::Template,
        |c| {
            let parts: Vec<&str> = c.get(1).unwrap().as_str().split('|').collect();
            let name = parts[0].trim().to_string();
            let params = parts.iter().skip(1).map(|p| p.trim().to_string()).collect();
            (name, params)
        },
        &mut earliest_match,
    );
    check_regex(
        text,
        get_bold_star_regex(),
        MarkdownTokenType::Bold,
        |c| (c.get(1).unwrap().as_str().to_string(), vec![]),
        &mut earliest_match,
    );
    check_regex(
        text,
        get_bold_underscore_regex(),
        MarkdownTokenType::Bold,
        |c| (c.get(1).unwrap().as_str().to_string(), vec![]),
        &mut earliest_match,
    );
    check_regex(
        text,
        get_italic_star_regex(),
        MarkdownTokenType::Italic,
        |c| (c.get(1).unwrap().as_str().to_string(), vec![]),
        &mut earliest_match,
    );
    check_regex(
        text,
        get_italic_underscore_regex(),
        MarkdownTokenType::Italic,
        |c| (c.get(1).unwrap().as_str().to_string(), vec![]),
        &mut earliest_match,
    );
    check_regex(
        text,
        get_link_regex(),
        MarkdownTokenType::Link,
        |c| {
            (
                c.get(2).unwrap().as_str().to_string(),
                vec![c.get(1).unwrap().as_str().to_string()],
            )
        },
        &mut earliest_match,
    );

    if let Some((start, end, token_type, mut payload)) = earliest_match {
        if start > 0 {
            tokens.push(MarkdownToken::new(MarkdownTokenType::Text, &text[..start]));
        }

        let main_val = payload.remove(0);
        tokens.push(MarkdownToken {
            token_type,
            level: 0,
            value: main_val,
            parameters: payload,
        });

        if end < text.len() {
            tokenize_inline(&text[end..], tokens);
        }
    } else {
        tokens.push(MarkdownToken::new(MarkdownTokenType::Text, text));
    }
}

pub fn to_html(markdown: &str) -> String {
    if markdown.trim().is_empty() {
        return String::new();
    }

    let processed = crate::scripting::process_rhai_macros(markdown);
    let tokens = tokenize(&processed);
    render_tokens_to_html(&tokens)
}

pub fn get_categories(markdown: &str) -> Vec<String> {
    if markdown.trim().is_empty() {
        return Vec::new();
    }

    let tokens = tokenize(markdown);
    tokens
        .into_iter()
        .filter(|t| t.token_type == MarkdownTokenType::Category)
        .map(|t| t.value)
        .collect()
}

fn render_tokens_to_html(tokens: &[MarkdownToken]) -> String {
    let mut html = String::new();
    let mut i = 0;

    while i < tokens.len() {
        let token = &tokens[i];

        match token.token_type {
            MarkdownTokenType::Heading => {
                html.push_str(&format!("<h{}>", token.level));
                let mut sub_tokens = Vec::new();
                tokenize_inline(&token.value, &mut sub_tokens);
                html.push_str(&render_inline_tokens(&sub_tokens));
                html.push_str(&format!("</h{}>\n", token.level));
            }
            MarkdownTokenType::CodeBlock => {
                html.push_str("<pre><code>");
                html.push_str(&html_escape::encode_text(&token.value));
                html.push_str("</code></pre>\n");
            }
            MarkdownTokenType::Template => {
                html.push_str(&format!(
                    "<div class=\"markdown-template\" data-name=\"{}\">",
                    html_escape::encode_double_quoted_attribute(&token.value)
                ));
                html.push_str(&html_escape::encode_text(&token.parameters.join(", ")));
                html.push_str("</div>");
            }
            MarkdownTokenType::List => {
                html.push_str("<ul>\n");
                while i < tokens.len() && tokens[i].token_type == MarkdownTokenType::List {
                    html.push_str("<li>");
                    let mut sub_tokens = Vec::new();
                    tokenize_inline(&tokens[i].value, &mut sub_tokens);
                    html.push_str(&render_inline_tokens(&sub_tokens));
                    html.push_str("</li>\n");
                    i += 1;
                }
                html.push_str("</ul>\n");
                continue;
            }
            MarkdownTokenType::TableRow => {
                html.push_str("<table>\n");
                while i < tokens.len()
                    && (tokens[i].token_type == MarkdownTokenType::TableRow
                        || tokens[i].token_type == MarkdownTokenType::TableDivider
                        || tokens[i].token_type == MarkdownTokenType::Newline)
                {
                    if tokens[i].token_type == MarkdownTokenType::TableRow {
                        html.push_str("<tr>\n");
                        let cells: Vec<&str> = tokens[i]
                            .value
                            .split('|')
                            .filter(|s| !s.is_empty())
                            .collect();
                        for cell in cells {
                            html.push_str("<td>");
                            let mut sub_tokens = Vec::new();
                            tokenize_inline(cell.trim(), &mut sub_tokens);
                            html.push_str(&render_inline_tokens(&sub_tokens));
                            html.push_str("</td>");
                        }
                        html.push_str("</tr>\n");
                    }
                    i += 1;
                }
                html.push_str("</table>\n");
                continue;
            }
            MarkdownTokenType::Category => {
                // Categories are omitted from HTML
            }
            _ => {
                // Paragraph wrapping for inline tokens
                if is_inline(&token.token_type) {
                    let mut p_tokens = Vec::new();
                    while i < tokens.len() {
                        if tokens[i].token_type == MarkdownTokenType::Newline {
                            if i + 1 < tokens.len()
                                && tokens[i + 1].token_type == MarkdownTokenType::Newline
                            {
                                i += 1;
                                break;
                            }
                            let has_more =
                                (i + 1..tokens.len()).any(|k| is_inline(&tokens[k].token_type));
                            if has_more {
                                p_tokens.push(MarkdownToken::new(MarkdownTokenType::Text, "\n"));
                            }
                        } else if is_inline(&tokens[i].token_type) {
                            p_tokens.push(tokens[i].clone());
                        } else {
                            break;
                        }
                        i += 1;
                    }
                    if !p_tokens.is_empty() {
                        html.push_str("<p>");
                        html.push_str(&render_inline_tokens(&p_tokens));
                        html.push_str("</p>\n");
                    }
                    continue;
                }
            }
        }
        i += 1;
    }

    html
}

fn is_inline(token_type: &MarkdownTokenType) -> bool {
    matches!(
        token_type,
        MarkdownTokenType::Text
            | MarkdownTokenType::Bold
            | MarkdownTokenType::Italic
            | MarkdownTokenType::Link
            | MarkdownTokenType::Category
            | MarkdownTokenType::CodeInline
    )
}

fn render_inline_tokens(tokens: &[MarkdownToken]) -> String {
    let mut out = String::new();
    for token in tokens {
        match token.token_type {
            MarkdownTokenType::Text => {
                let escaped = html_escape::encode_text(&token.value);
                out.push_str(&escaped.replace('\n', "<br />\n"));
            }
            MarkdownTokenType::Bold => {
                out.push_str("<strong>");
                let mut sub = Vec::new();
                tokenize_inline(&token.value, &mut sub);
                out.push_str(&render_inline_tokens(&sub));
                out.push_str("</strong>");
            }
            MarkdownTokenType::Italic => {
                out.push_str("<em>");
                let mut sub = Vec::new();
                tokenize_inline(&token.value, &mut sub);
                out.push_str(&render_inline_tokens(&sub));
                out.push_str("</em>");
            }
            MarkdownTokenType::Link => {
                let label = if !token.parameters.is_empty() {
                    &token.parameters[0]
                } else {
                    &token.value
                };
                out.push_str(&format!(
                    "<a href=\"{}\">{}</a>",
                    html_escape::encode_double_quoted_attribute(&token.value),
                    html_escape::encode_text(label)
                ));
            }
            MarkdownTokenType::CodeInline => {
                out.push_str("<code>");
                out.push_str(&html_escape::encode_text(&token.value));
                out.push_str("</code>");
            }
            MarkdownTokenType::Template => {
                out.push_str(&format!(
                    "<div class=\"markdown-template\" data-name=\"{}\">{}</div>",
                    html_escape::encode_double_quoted_attribute(&token.value),
                    html_escape::encode_text(&token.parameters.join(", "))
                ));
            }
            MarkdownTokenType::Category => {}
            _ => {}
        }
    }
    out
}
