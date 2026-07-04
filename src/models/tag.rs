/// Normalize a raw tag string.
/// - Convert to lowercase
/// - Replace spaces with hyphens
/// - Trim whitespace
pub fn normalize(tag: &str) -> String {
    tag.trim().to_lowercase().replace(' ', "-")
}

/// Parse a comma-separated tag string into a vector of normalized tags.
pub fn parse_tags(input: &str) -> Vec<String> {
    input
        .split(',')
        .map(normalize)
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        assert_eq!(normalize("Docker"), "docker");
        assert_eq!(normalize("  JavaScript  "), "javascript");
        assert_eq!(normalize("machine learning"), "machine-learning");
    }

    #[test]
    fn test_parse_tags() {
        assert_eq!(
            parse_tags("Docker, JavaScript, Machine Learning"),
            vec!["docker", "javascript", "machine-learning"]
        );
    }
}
