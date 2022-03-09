pub fn assert_response(rules: Vec<AssertRule>, response: String) -> Vec<AssertResult> {
    rules
        .iter()
        .map(|rule| match rule {
            AssertRule::ExactMatch(str) => assert_exact_match(str, &response),
        })
        .collect()
}

fn assert_exact_match(str: &String, response: &String) -> AssertResult {
    if response.contains(str) {
        AssertResult::Success
    } else {
        AssertResult::Failure(format!("Expected response to contain '{}'", str))
    }
}

#[derive(Debug, PartialEq)]
pub enum AssertResult {
    Success,
    Failure(String),
}

pub enum AssertRule {
    ExactMatch(String),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_assets_exact_match() {
        assert_eq!(
            assert_response(
                vec![AssertRule::ExactMatch("foo".to_string())],
                "barr foo".to_string()
            ),
            vec![AssertResult::Success]
        );

        assert_eq!(
            assert_response(
                vec![AssertRule::ExactMatch("foo".to_string())],
                "barr foeo".to_string()
            ),
            vec![AssertResult::Failure("Expected response to contain 'foo'".to_string())]
        );
    }
}
