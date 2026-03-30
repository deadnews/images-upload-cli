use anyhow::{Context, Result};
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use tracing::debug;

use super::response_text;

pub const API_URL: &str = "https://imagebin.ca/upload.php";

/// Upload image bytes to imagebin.ca.
///
/// No authentication required. Returns plain text with `url:` field.
pub async fn upload(client: &Client, data: Vec<u8>, url: &str) -> Result<String> {
    let form = Form::new().part("file", Part::bytes(data).file_name("image"));

    let resp = client
        .post(url)
        .multipart(form)
        .send()
        .await
        .context("failed to send request to imagebin")?;

    let body = response_text(resp, "imagebin").await?;

    for line in body.lines() {
        if let Some(url) = line.strip_prefix("url:") {
            return Ok(url.trim().to_owned());
        }
    }

    debug!("Response text:\n{body}");
    anyhow::bail!("url not found in imagebin response")
}

#[cfg(test)]
#[expect(clippy::unwrap_used)]
mod tests {
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    #[tokio::test]
    async fn test_upload_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("status:ok\nurl:https://ibin.co/7QknLCJgP5iN.png"),
            )
            .mount(&mock_server)
            .await;

        let client = Client::new();
        let url = upload(&client, vec![1, 2, 3], &mock_server.uri())
            .await
            .unwrap();
        assert_eq!(url, "https://ibin.co/7QknLCJgP5iN.png");
    }
}
