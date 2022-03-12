mod response_asserter;

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Error, Method,
};
use std::time::Instant;

use crate::{Method as CreateMethod, TestCase};

pub fn create_client() -> reqwest::Client {
    reqwest::Client::new()
}

pub async fn make_test_case_request(
    test_case_config: &TestCase,
    client: &reqwest::Client,
) -> Result<TestCaseRunResult, Error> {
    let now = Instant::now();

    let mut header_map = HeaderMap::new();

    for (key, value) in test_case_config.headers.iter() {
        header_map.insert(
            HeaderName::from_bytes(key.as_bytes()).unwrap(),
            HeaderValue::from_str(value.as_str()).unwrap(),
        );
    }

    let request = client
        .request(
            normalize_method(&test_case_config.method),
            &test_case_config.url,
        )
        .headers(header_map)
        .body(test_case_config.data.clone())
        .build()?;

    let response = client.execute(request).await?;

    let status = response.status();

    let body = response.text().await?;

    let assers_results =
        response_asserter::assert_response(&test_case_config.assertions, body.clone());

    Ok(TestCaseRunResult {
        status: status.as_u16(),
        time: now.elapsed().as_millis() as f32 / 1000.0,
        assert_results: assers_results,
        body,
    })
}

fn normalize_method(method: &CreateMethod) -> Method {
    match method {
        CreateMethod::Options => Method::OPTIONS,
        CreateMethod::Get => Method::GET,
        CreateMethod::Post => Method::POST,
        CreateMethod::Put => Method::PUT,
        CreateMethod::Delete => Method::DELETE,
        CreateMethod::Head => Method::HEAD,
        CreateMethod::Trace => Method::TRACE,
        CreateMethod::Connect => Method::CONNECT,
        CreateMethod::Patch => Method::PATCH,
    }
}

#[derive(Debug)]
pub struct TestCaseRunResult {
    pub status: u16,
    pub time: f32,
    pub assert_results: String,
    pub body: String,
}
