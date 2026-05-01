pub(crate) fn is_valid_json_no_alloc(input: &str) -> bool {
    fixed_json::validate_json(input.as_bytes()).is_ok()
}

pub(crate) fn looks_like_json_object(input: &str) -> bool {
    input.trim_start().starts_with('{')
}

#[cfg(test)]
mod tests {
    use super::is_valid_json_no_alloc;

    #[test]
    fn validates_json_without_value_allocation() {
        assert!(is_valid_json_no_alloc(r#"{"query":"status","limit":3}"#));
        assert!(is_valid_json_no_alloc(r#"["a",true,null]"#));
        assert!(!is_valid_json_no_alloc(r#"{"query":"status",}"#));
    }

    #[test]
    fn detects_probable_json_object_without_parsing() {
        assert!(super::looks_like_json_object("  {\"tool_calls\":[]}"));
        assert!(!super::looks_like_json_object("normal assistant reply"));
        assert!(!super::looks_like_json_object(
            "[{\"not\":\"history object\"}]"
        ));
    }
}
