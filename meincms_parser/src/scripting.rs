use regex::Regex;
use rhai::{Engine, Scope};

/// Führt ein Rhai-Skript in einer abgesicherten Sandbox aus.
pub fn eval_rhai_script(script: &str) -> Result<String, String> {
    let mut engine = Engine::new();

    // Sandbox-Sicherheitslimits setzen gegen Endlosschleifen & Overflow
    engine.set_max_operations(5000);
    engine.set_max_expr_depths(20, 20);

    let mut scope = Scope::new();

    match engine.eval_with_scope::<rhai::Dynamic>(&mut scope, script) {
        Ok(result) => Ok(result.to_string()),
        Err(err) => Err(format!("Rhai Fehler: {}", err)),
    }
}

/// Ersetzt alle {{#rhai: ...}} oder {{#script: ...}} Makros im Text durch das Skript-Ergebnis
pub fn process_rhai_macros(text: &str) -> String {
    let re = Regex::new(r"\{\{#(?:rhai|script):\s*(.*?)\}\}").unwrap();
    re.replace_all(text, |caps: &regex::Captures| {
        let code = caps.get(1).map_or("", |m| m.as_str());
        match eval_rhai_script(code) {
            Ok(val) => val,
            Err(err) => format!(
                "<span style=\"color: red;\">[{}]</span>",
                html_escape::encode_text(&err)
            ),
        }
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_simple_math() {
        let res = eval_rhai_script("5 * 10 + 2");
        assert_eq!(res, Ok("52".to_string()));
    }

    #[test]
    fn test_eval_string_concat() {
        let res = eval_rhai_script(r#""Hallo " + "Welt!""#);
        assert_eq!(res, Ok("Hallo Welt!".to_string()));
    }

    #[test]
    fn test_macro_replacement() {
        let input = "Das Ergebnis ist: {{#rhai: 7 * 8}}!";
        let output = process_rhai_macros(input);
        assert_eq!(output, "Das Ergebnis ist: 56!");
    }

    #[test]
    fn test_sandbox_infinite_loop_protection() {
        let input = "{{#rhai: loop { } }}";
        let output = process_rhai_macros(input);
        assert!(output.contains("Rhai Fehler"));
    }
}
