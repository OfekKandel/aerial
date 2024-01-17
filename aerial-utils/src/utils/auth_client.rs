use reqwest::blocking::RequestBuilder;

pub trait AuthClient {
    fn add_auth(&self, request: RequestBuilder) -> RequestBuilder;
}

pub trait AddAuthExt {
    fn auth(self, client: &impl AuthClient) -> Self;
}

impl AddAuthExt for RequestBuilder {
    fn auth(self, client: &impl AuthClient) -> Self {
        client.add_auth(self)
    }
}
