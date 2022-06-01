use std::collections::HashMap;

use reqwest::{
    blocking::{Client, Response},
    Method, Url,
};
use serde;

use crate::common::errors::HttpError;

#[derive(Debug)]
pub struct HttpClient {
    base_url: Url,
    client: Client,
    timeout: u64,
}

impl HttpClient {
    pub fn new(timeout: u64) -> Result<HttpClient, HttpError> {
        if timeout < 1 {
            return Err(HttpError::InvalidTimeout(timeout));
        }

        let base_url = Url::parse("https://api.ng.termii.com/api/")
            .map_err(|err| HttpError::UrlParseError(err.to_string()))?;

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(timeout))
            .build()?;

        Ok(HttpClient {
            base_url: base_url,
            client: client,
            timeout: timeout,
        })
    }

    fn request<T>(
        &self,
        url: &str,
        method: Method,
        params: Option<HashMap<&str, &str>>,
        _headers: Option<HashMap<&str, &str>>,
        data: Option<T>,
    ) -> Result<Response, HttpError>
    where
        T: serde::Serialize,
    {
        let timeout = Some(self.timeout);

        let url = self
            .base_url
            .join(url)
            .map_err(|err| HttpError::UrlParseError(err.to_string()))?;

        let mut _client = self
            .client
            .request(method, url)
            .timeout(std::time::Duration::from_millis(timeout.unwrap() * 1000));

        if let Some(params) = params {
            _client = _client.query(&params);
        }

        if let Some(data) = data {
            _client = _client.json(&data);
        }

        let response = _client.send()?;

        Ok(response)
    }

    pub fn get(
        &self,
        url: &str,
        params: Option<HashMap<&str, &str>>,
        headers: Option<HashMap<&str, &str>>,
    ) -> Result<Response, HttpError> {
        self.request(url, Method::GET, params, headers, None::<u8>)
    }

    pub fn post<T>(
        &self,
        url: &str,
        params: Option<HashMap<&str, &str>>,
        headers: Option<HashMap<&str, &str>>,
        data: Option<T>,
    ) -> Result<Response, HttpError>
    where
        T: serde::Serialize,
    {
        self.request(url, Method::POST, params, headers, data)
    }

    pub fn patch<T>(
        &self,
        url: &str,
        params: Option<HashMap<&str, &str>>,
        headers: Option<HashMap<&str, &str>>,
        data: Option<T>,
    ) -> Result<Response, HttpError>
    where
        T: serde::Serialize,
    {
        self.request(url, Method::PATCH, params, headers, data)
    }

    pub fn delete(
        &self,
        url: &str,
        params: Option<HashMap<&str, &str>>,
        headers: Option<HashMap<&str, &str>>,
    ) -> Result<Response, HttpError> {
        self.request(url, Method::DELETE, params, headers, None::<u8>)
    }
}
