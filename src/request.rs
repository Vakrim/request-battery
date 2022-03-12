mod response_asserter;

use reqwest::Error;
use std::time::Instant;

use crate::TestCase;

pub async fn make_test_case_request(
    test_case_config: &TestCase,
) -> Result<TestCaseRunResult, Error> {
    let now = Instant::now();
    let response = reqwest::get(test_case_config.url.clone()).await?;

    let status = response.status();

    let assers_results =
        response_asserter::assert_response(&test_case_config.assertions, response.text().await?);

    Ok(TestCaseRunResult {
        status: status.as_u16(),
        time: now.elapsed().as_millis() as f32 / 1000.0,
        assert_results: assers_results,
    })
}

#[derive(Debug)]
pub struct TestCaseRunResult {
    pub status: u16,
    pub time: f32,
    pub assert_results: String,
}
