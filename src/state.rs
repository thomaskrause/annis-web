#[cfg(test)]
use mockito;
use url::Url;

use crate::Result;

pub struct GlobalAppState {
    pub service_url: Url,
}

impl GlobalAppState {
    pub fn new() -> Result<Self> {
        #[cfg(not(test))]
        // TODO: get this parameter a configuration
        let service_url = "http://localhost:5711/v1/";

        #[cfg(test)]
        let service_url: &str = &mockito::server_url();

        let result = Self {
            service_url: Url::parse(service_url)?,
        };
        Ok(result)
    }
}