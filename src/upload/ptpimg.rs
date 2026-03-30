use anyhow::{Context, Result};
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use super::parse_json;

pub const API_URL: &str = "https://ptpimg.me/upload.php";

#[derive(Deserialize)]
struct Entry {
    code: String,
    ext: String,
}

/// Upload image bytes to ptpimg.me.
///
/// Requires API key.
pub async fn upload(client: &Client, data: Vec<u8>, url: &str, key: &str) -> Result<String> {
    let form = Form::new()
        .text("api_key", key.to_owned())
        .part("file-upload[0]", Part::bytes(data).file_name("image"));

    let resp = client
        .post(url)
        .multipart(form)
        .send()
        .await
        .context("failed to send request to ptpimg")?;

    let entries: Vec<Entry> = parse_json(resp, "ptpimg").await?;

    let entry = entries
        .into_iter()
        .next()
        .context("ptpimg returned no entries")?;
    Ok(format!("https://ptpimg.me/{}.{}", entry.code, entry.ext))
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
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"code": "8i531v", "ext": "png"}
            ])))
            .mount(&mock_server)
            .await;

        let client = Client::new();
        let url = upload(&client, vec![1, 2, 3], &mock_server.uri(), "test_key")
            .await
            .unwrap();
        assert_eq!(url, "https://ptpimg.me/8i531v.png");
    }
}
