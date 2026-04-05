use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

pub const CONTENT_URL: &str = "https://content.dropboxapi.com";
pub const API_URL: &str = "https://api.dropboxapi.com";

#[derive(Deserialize)]
struct UploadResponse {
    path_display: String,
}

#[derive(Deserialize)]
struct LinkResponse {
    url: String,
}

#[derive(Deserialize)]
struct ListLinksResponse {
    links: Vec<LinkResponse>,
}

/// Upload image bytes to Dropbox.
///
/// Requires access token with `files.content.write` and `sharing.write` permissions.
pub async fn upload(
    client: &Client,
    data: Vec<u8>,
    content_url: &str,
    api_url: &str,
    token: &str,
) -> Result<String> {
    let api_arg = json!({
        "path": "/image.png",
        "mode": "add",
        "autorename": true,
    });

    let resp = client
        .post(format!("{content_url}/2/files/upload"))
        .bearer_auth(token)
        .header("Content-Type", "application/octet-stream")
        .header("Dropbox-API-Arg", api_arg.to_string())
        .body(data)
        .send()
        .await
        .context("failed to upload to Dropbox")?;

    let uploaded: UploadResponse = super::parse_json(resp, "dropbox upload").await?;
    let shared_url = shared_link(client, api_url, token, &uploaded.path_display).await?;

    Ok(to_direct_link(&shared_url))
}

async fn shared_link(client: &Client, api_url: &str, token: &str, path: &str) -> Result<String> {
    let resp = client
        .post(format!(
            "{api_url}/2/sharing/create_shared_link_with_settings"
        ))
        .bearer_auth(token)
        .header("Content-Type", "application/json")
        .body(json!({"path": path, "settings": {"requested_visibility": "public"}}).to_string())
        .send()
        .await
        .context("failed to create Dropbox shared link")?;

    if resp.status() == 409 {
        return existing_shared_link(client, api_url, token, path).await;
    }

    Ok(super::parse_json::<LinkResponse>(resp, "dropbox share")
        .await?
        .url)
}

async fn existing_shared_link(
    client: &Client,
    api_url: &str,
    token: &str,
    path: &str,
) -> Result<String> {
    let resp = client
        .post(format!("{api_url}/2/sharing/list_shared_links"))
        .bearer_auth(token)
        .header("Content-Type", "application/json")
        .body(json!({"path": path, "direct_only": true}).to_string())
        .send()
        .await
        .context("failed to list Dropbox shared links")?;

    let listed: ListLinksResponse = super::parse_json(resp, "dropbox list links").await?;
    listed
        .links
        .into_iter()
        .next()
        .map(|l| l.url)
        .context("no shared link found for Dropbox file")
}

fn to_direct_link(url: &str) -> String {
    url.replace("www.dropbox.com", "dl.dropboxusercontent.com")
        .replace("&dl=0", "")
        .replace("?dl=0", "")
}

#[cfg(test)]
#[expect(clippy::unwrap_used)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    #[tokio::test]
    async fn test_upload_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/2/files/upload"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "name": "image.png",
                "path_display": "/image.png",
            })))
            .mount(&mock_server)
            .await;

        Mock::given(method("POST"))
            .and(path("/2/sharing/create_shared_link_with_settings"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "url": "https://www.dropbox.com/scl/fi/abc/image.png?rlkey=key&dl=0",
                ".tag": "file",
            })))
            .mount(&mock_server)
            .await;

        let base = mock_server.uri();
        let client = Client::new();
        let result = upload(&client, vec![1, 2, 3], &base, &base, "test_token")
            .await
            .unwrap();

        assert_eq!(
            result,
            "https://dl.dropboxusercontent.com/scl/fi/abc/image.png?rlkey=key"
        );
    }

    #[tokio::test]
    async fn test_upload_existing_link() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/2/files/upload"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "path_display": "/image.png",
            })))
            .mount(&mock_server)
            .await;

        Mock::given(method("POST"))
            .and(path("/2/sharing/create_shared_link_with_settings"))
            .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
                "error_summary": "shared_link_already_exists/..",
                "error": {".tag": "shared_link_already_exists"},
            })))
            .mount(&mock_server)
            .await;

        Mock::given(method("POST"))
            .and(path("/2/sharing/list_shared_links"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "links": [{"url": "https://www.dropbox.com/scl/fi/abc/image.png?rlkey=key&dl=0"}],
                "has_more": false,
            })))
            .mount(&mock_server)
            .await;

        let base = mock_server.uri();
        let client = Client::new();
        let result = upload(&client, vec![1, 2, 3], &base, &base, "test_token")
            .await
            .unwrap();

        assert_eq!(
            result,
            "https://dl.dropboxusercontent.com/scl/fi/abc/image.png?rlkey=key"
        );
    }

    #[test]
    fn test_to_direct_link() {
        let url = "https://www.dropbox.com/scl/fi/abc/image.png?rlkey=key&dl=0";
        assert_eq!(
            to_direct_link(url),
            "https://dl.dropboxusercontent.com/scl/fi/abc/image.png?rlkey=key"
        );
    }
}
