mod response_asserter;

use std::time::Instant;
use reqwest::Error;

pub async fn make_test_case_request(url: String) -> Result<TestCaseRunResult, Error> {
    let now = Instant::now();
    let response = reqwest::get(url).await?;

    let status = response.status();

    let assers_results = response_asserter::assert_response(vec![], response.text().await?);

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
