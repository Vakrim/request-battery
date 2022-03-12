use crate::Assertion;

pub fn assert_response(rules: &Vec<Assertion>, response: String) -> String {
    let results = execute_assertrions(rules, response);
    return format_assert_response(results);
}

fn format_assert_response(results: Vec<AssertResult>) -> String {
    let errors = results
        .iter()
        .filter_map(|r| -> Option<String> {
            if let AssertResult::Failure(reason) = r {
                Some(reason.clone())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    if errors.len() == 0 {
        return format!("Assertions passed ({})", results.len());
    }

    let mut failed_message = "Assertions failed: ".to_string();
    failed_message.push_str(&errors.join(", "));
    return failed_message;
}

fn execute_assertrions(rules: &Vec<Assertion>, response: String) -> Vec<AssertResult> {
    rules
        .iter()
        .map(|rule| match rule {
            Assertion::ExactMatch(str) => assert_exact_match(str, &response),
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

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn it_assets_exact_match() {
        assert_eq!(
            execute_assertrions(
                &vec![Assertion::ExactMatch("foo".to_string())],
                "barr foo".to_string()
            ),
            vec![AssertResult::Success]
        );

        assert_eq!(
            execute_assertrions(
                &vec![Assertion::ExactMatch("foo".to_string())],
                "barr foeo".to_string()
            ),
            vec![AssertResult::Failure(
                "Expected response to contain 'foo'".to_string()
            )]
        );
    }

    #[test]
    fn it_formats_assertion_errors() {
        assert_eq!(
          assert_response(
              &vec![
                  Assertion::ExactMatch("foo".to_string()),
                  Assertion::ExactMatch("bar".to_string())
              ],
              "some response body".to_string()
          ),
          "Assertions failed: Expected response to contain 'foo', Expected response to contain 'bar'"
      );

        assert_eq!(
            assert_response(
                &vec![
                    Assertion::ExactMatch("foo".to_string()),
                    Assertion::ExactMatch("bar".to_string())
                ],
                "some fooo response bar body".to_string()
            ),
            "Assertions passed (2)"
        );
    }
}
