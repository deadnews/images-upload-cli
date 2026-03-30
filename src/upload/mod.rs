mod beeimg;
mod catbox;
mod fastpic;
mod freeimage;
mod gyazo;
mod imageban;
mod imagebin;
mod imgbb;
mod imgchest;
mod imgur;
mod lensdump;
mod pixeldrain;
mod pixhost;
mod ptpimg;
mod sxcu;
mod thumbsnap;
mod tixte;
mod uplio;
mod uploadcare;
mod vgy;
mod zpic;

use anyhow::{Context, Result, ensure};
use clap::ValueEnum;
use reqwest::Client;
use serde::de::DeserializeOwned;
use tracing::debug;

use crate::util::get_env;

/// Parse a JSON response, checking status and logging the body at debug level on failure.
pub(crate) async fn parse_json<T: DeserializeOwned>(
    resp: reqwest::Response,
    provider: &str,
) -> Result<T> {
    let body = response_text(resp, provider).await?;
    serde_json::from_str(&body)
        .inspect_err(|_| debug!("Response text:\n{body}"))
        .with_context(|| format!("failed to parse {provider} response"))
}

/// Read response text, checking status first.
pub(crate) async fn response_text(resp: reqwest::Response, provider: &str) -> Result<String> {
    let status = resp.status();
    let body = resp
        .text()
        .await
        .with_context(|| format!("failed to read {provider} response"))?;

    if !status.is_success() {
        anyhow::bail!("{provider} returned {status}: {body}");
    }

    Ok(body)
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Hosting {
    Beeimg,
    Catbox,
    Fastpic,
    Freeimage,
    Gyazo,
    Imageban,
    Imagebin,
    Imgbb,
    Imgchest,
    Imgur,
    Lensdump,
    Pixeldrain,
    Pixhost,
    Ptpimg,
    Sxcu,
    Thumbsnap,
    Tixte,
    Uplio,
    Uploadcare,
    Vgy,
    Zpic,
}

impl std::fmt::Display for Hosting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            self.to_possible_value()
                .expect("all variants have values")
                .get_name(),
        )
    }
}

/// Dispatch upload to the appropriate provider.
pub async fn upload(client: &Client, hosting: Hosting, data: Vec<u8>) -> Result<String> {
    let url = match hosting {
        Hosting::Beeimg => beeimg::upload(client, data, beeimg::API_URL).await,
        Hosting::Catbox => catbox::upload(client, data, catbox::API_URL).await,
        Hosting::Fastpic => fastpic::upload(client, data, fastpic::API_URL).await,
        Hosting::Imagebin => imagebin::upload(client, data, imagebin::API_URL).await,
        Hosting::Pixhost => pixhost::upload(client, data, pixhost::API_URL).await,
        Hosting::Sxcu => sxcu::upload(client, data, sxcu::API_URL).await,
        Hosting::Freeimage => {
            let key = get_env("FREEIMAGE_KEY")?;
            freeimage::upload(client, data, freeimage::API_URL, &key).await
        }
        Hosting::Gyazo => {
            let token = get_env("GYAZO_TOKEN")?;
            gyazo::upload(client, data, gyazo::API_URL, &token).await
        }
        Hosting::Imageban => {
            let token = get_env("IMAGEBAN_TOKEN")?;
            imageban::upload(client, data, imageban::API_URL, &token).await
        }
        Hosting::Imgbb => {
            let key = get_env("IMGBB_KEY")?;
            imgbb::upload(client, data, imgbb::API_URL, &key).await
        }
        Hosting::Imgchest => {
            let key = get_env("IMGCHEST_KEY")?;
            imgchest::upload(client, data, imgchest::API_URL, &key).await
        }
        Hosting::Imgur => {
            let client_id = std::env::var("IMGUR_CLIENT_ID")
                .unwrap_or_else(|_| imgur::DEFAULT_CLIENT_ID.to_owned());
            imgur::upload(client, data, imgur::API_URL, &client_id).await
        }
        Hosting::Lensdump => {
            let key = get_env("LENSDUMP_KEY")?;
            lensdump::upload(client, data, lensdump::API_URL, &key).await
        }
        Hosting::Pixeldrain => {
            let key = get_env("PIXELDRAIN_KEY")?;
            pixeldrain::upload(client, data, pixeldrain::API_URL, &key).await
        }
        Hosting::Ptpimg => {
            let key = get_env("PTPIMG_KEY")?;
            ptpimg::upload(client, data, ptpimg::API_URL, &key).await
        }
        Hosting::Thumbsnap => {
            let key = get_env("THUMBSNAP_KEY")?;
            thumbsnap::upload(client, data, thumbsnap::API_URL, &key).await
        }
        Hosting::Tixte => {
            let key = get_env("TIXTE_KEY")?;
            tixte::upload(client, data, tixte::API_URL, &key).await
        }
        Hosting::Uplio => {
            let key = get_env("UPLIO_KEY")?;
            uplio::upload(client, data, uplio::API_URL, &key).await
        }
        Hosting::Uploadcare => {
            let key = get_env("UPLOADCARE_KEY")?;
            uploadcare::upload(client, data, uploadcare::API_URL, &key).await
        }
        Hosting::Vgy => {
            let key = get_env("VGY_KEY")?;
            vgy::upload(client, data, vgy::API_URL, &key).await
        }
        Hosting::Zpic => {
            let key = get_env("ZPIC_KEY")?;
            zpic::upload(client, data, zpic::API_URL, &key).await
        }
    }?;

    ensure!(!url.is_empty(), "{hosting} returned empty URL");
    Ok(url)
}
