use crate::{HttpRequest, HttpRequestMethod, Request};

impl Request {
    /// Sends the request and returns the response using default client.
    pub fn send(&self) -> Result<crate::response::Response, reqwest::Error> {
        HttpRequest::from(self).send()
    }

    /// Sends the request using the provided client.
    pub fn send_with_client(
        &self,
        client: &reqwest::Client,
    ) -> Result<crate::response::Response, reqwest::Error> {
        HttpRequest::from(self).send_with_client(client)
    }
}

impl HttpRequest {
    /// Sends the request and returns the response using default client.
    pub fn send(&self) -> Result<crate::response::Response, reqwest::Error> {
        self.send_with_client(&reqwest::Client::new())
    }

    /// Sends the request using the provided client.
    pub fn send_with_client(
        &self,
        client: &reqwest::Client,
    ) -> Result<crate::response::Response, reqwest::Error> {
        let request = match self.method {
            HttpRequestMethod::Get => client.get(&self.url).query(&self.values),
            HttpRequestMethod::Post => unimplemented!(),
        };
        request.send()?.json()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_request_without_api_key() {
        assert!(super::Request::from("Hello this grammarly world!")
            .send()
            .is_ok());
    }
}
