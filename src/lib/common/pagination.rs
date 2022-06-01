use async_trait::async_trait;

use crate::common::errors;

pub trait PaginatedResource {
    type Item;

    fn _get(&self, page: &str) -> Result<Vec<Self::Item>, errors::HttpError>;

    fn all(&self) -> Result<Vec<Self::Item>, errors::HttpError> {
        let mut current_page = 1usize;
        let mut result: Vec<Self::Item> = Vec::new();

        loop {
            let response = self._get(current_page.to_string().as_str())?;

            if response.len() == 0 {
                break;
            }
            result.extend(response);

            current_page += 1;
        }

        Ok(result)
    }
}

#[async_trait]
pub trait PaginatedResourceAsync {
    type Item: Send;

    async fn _get(&self, page: &str) -> Result<Vec<Self::Item>, errors::HttpError>;

    async fn all(&self) -> Result<Vec<Self::Item>, errors::HttpError> {
        let mut current_page = 1usize;
        let mut result: Vec<Self::Item> = Vec::new();

        loop {
            let response = self._get(current_page.to_string().as_str()).await?;

            if response.len() == 0 {
                break;
            }
            result.extend(response);

            current_page += 1;
        }

        Ok(result)
    }
}
