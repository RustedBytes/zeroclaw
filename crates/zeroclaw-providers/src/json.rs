pub(crate) fn is_valid_json_no_alloc(input: &str) -> bool {
    fixed_json::validate_json(input.as_bytes()).is_ok()
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
}
