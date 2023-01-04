//! Utility functions useful for importing different recipe formats.

use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value;

use crate::SousError;

static JSON_LD_HTML: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"<script type="application/ld\+json">([\s\S]+)</script>"#).unwrap());

/// Extract a JSON-LD object from the provided HTML string.
pub fn extract_json(source: &str) -> Result<Value, SousError> {
    let capture = &JSON_LD_HTML
        .captures(source)
        .ok_or_else(|| SousError::NoSchemaFoundError())?[1];

    let json: Value = serde_json::from_str(capture)?;
    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_schema() {
        let html = r#"
<script type="application/ld+json">
    {
      "@context": "https://schema.org",
      "@type": "Recipe",
      "author": "Test Author"
    }
</script>
        "#;

        let json = extract_json(html).unwrap();
        println!("{:?}", json);
    }
}
