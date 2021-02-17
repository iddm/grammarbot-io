use crate::{HttpRequest, Request};

impl Request {
    /// Sends the request and returns the response using default client.
    pub fn send(&self) -> Result<crate::response::Response, reqwest::Error> {
        HttpRequest::from(self).send()
    }

    /// Sends the request using the provided client.
    pub fn send_with_client(
        &self,
        client: &reqwest::blocking::Client,
    ) -> Result<crate::response::Response, reqwest::Error> {
        HttpRequest::from(self).send_with_client(client)
    }
}

impl HttpRequest {
    /// Sends the request and returns the response using default client.
    pub fn send(&self) -> Result<crate::response::Response, reqwest::Error> {
        self.send_with_client(&reqwest::blocking::Client::new())
    }

    /// Sends the request using the provided client.
    pub fn send_with_client(
        &self,
        client: &reqwest::blocking::Client,
    ) -> Result<crate::response::Response, reqwest::Error> {
        let mut request = client.post(&self.url);
        for (k, v) in &self.headers {
            request = request.header(k, v);
        }
        request.form(&self.data).send()?.json()
    }
}
