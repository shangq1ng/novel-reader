use crate::error::s3::S3Error;
use anyhow::Context;
use s3::creds::Credentials;
use s3::{Bucket, Region};
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct S3ClientConfig {
    pub bucket: Bucket,
}

#[derive(Clone, Debug)]
pub struct S3Config {
    pub bucket: String,
    pub endpoint: String,
    pub region: String,
    pub key_id: String,
    pub application_key: String,
    pub path_style: bool,
    pub request_timeout: Duration,
}

impl S3Config {
    pub async fn from_env() -> Result<Self, S3Error> {
        Ok(Self {
            bucket: env::var("BUCKET_NAME").context("Set up BUCKET in .env")?,
            endpoint: env::var("ENDPOINT").context("Set up ENDPOINT in .env")?,
            region: env::var("REGION_NAME").context("Set up REGION_NAME in .env")?,
            key_id: env::var("KEY_ID").context("Set up KEY_ID in .env")?,
            application_key: env::var("APPLICATION_KEY")
                .context("Set up APPLICATION_KEY in .env")?,
            path_style: env::var("PATH_STYLE")?
                .parse::<bool>()
                .context("Setup PATH_STYLE in .env")?,
            request_timeout: Duration::from_secs(
                env::var("REQUEST_TIMEOUT")?
                    .parse::<u64>()
                    .context("Invalid REQUEST_TIMEOUT")?,
            ),
        })
    }
}

impl S3ClientConfig {
    pub async fn new(&self) -> Result<Self, S3Error> {
        let s3_config = S3Config::from_env().await?;

        let credential = Credentials::new(
            Some(&s3_config.key_id.as_str()),
            Some(&s3_config.application_key.as_str()),
            None,
            None,
            None,
        )
        .ok()
        .unwrap_or(Credentials::default().unwrap());

        let region: Region = s3_config.region.parse().context("Invalid region")?;

        let mut bucket = Bucket::new(&s3_config.bucket, region, credential)?;

        if s3_config.path_style {
            bucket = bucket.with_path_style()
        }

        bucket = bucket.with_request_timeout(s3_config.request_timeout)?;

        Ok(Self { bucket: *bucket })
    }
}
