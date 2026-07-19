pub mod ffi;
pub mod markdown;
pub mod scripting;
pub mod wikitext;

pub use markdown::{get_categories as get_markdown_categories, to_html as markdown_to_html};
pub use scripting::{eval_rhai_script, process_rhai_macros};
pub use wikitext::{get_categories as get_wikitext_categories, to_html as wikitext_to_html};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading1() {
        let result = markdown_to_html("# Headline");
        assert_eq!(result, "<h1>Headline</h1>\n");
    }

    #[test]
    fn test_bold_and_italic() {
        let result = markdown_to_html("**Bold** and *Italic*");
        assert_eq!(result, "<p><strong>Bold</strong> and <em>Italic</em></p>\n");
    }

    #[test]
    fn test_link() {
        let result = markdown_to_html("[Google](https://google.com)");
        assert_eq!(result, "<p><a href=\"https://google.com\">Google</a></p>\n");
    }

    #[test]
    fn test_template() {
        let result = markdown_to_html("{{Info|Important Message}}");
        assert_eq!(
            result,
            "<div class=\"markdown-template\" data-name=\"Info\">Important Message</div>"
        );
    }

    #[test]
    fn test_list() {
        let result = markdown_to_html("* Item 1\n* Item 2");
        assert!(result.contains("<li>Item 1</li>"));
        assert!(result.contains("<li>Item 2</li>"));
        assert!(result.contains("<ul>"));
    }

    #[test]
    fn test_category_invisible() {
        let result = markdown_to_html("Hallo Welt [[kategorie:Hauptseite]]");
        assert!(result.contains("Hallo Welt"));
        assert!(!result.contains("Hauptseite"));
    }

    #[test]
    fn test_code() {
        let inline_result = markdown_to_html("Benutze `var x = 1;`!");
        assert_eq!(inline_result, "<p>Benutze <code>var x = 1;</code>!</p>\n");

        let block_result = markdown_to_html("```\nConsole.WriteLine();\n```");
        assert_eq!(
            block_result,
            "<pre><code>Console.WriteLine();</code></pre>\n"
        );

        let special_result = markdown_to_html("'''\nSpecial Code\n'''");
        assert_eq!(special_result, "<pre><code>Special Code</code></pre>\n");
    }

    #[test]
    fn test_rhai_script_macro_in_markdown() {
        let result = markdown_to_html("Berechnung: {{#rhai: 10 + 20}}");
        assert_eq!(result, "<p>Berechnung: 30</p>\n");
    }

    #[test]
    fn test_get_categories() {
        let categories = get_markdown_categories("[[kategorie:A]]\n[[Kategorie:B]]");
        assert_eq!(categories.len(), 2);
        assert!(categories.contains(&"A".to_string()));
        assert!(categories.contains(&"B".to_string()));
    }
}
