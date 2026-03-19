use networking_stack::http1_client::{HttpClient, NativeHttp1Client, Response};

#[derive(Debug, Default)]
pub struct ResourceLoader {
    client: NativeHttp1Client,
}

impl ResourceLoader {
    pub fn load(&self, url: &str) -> Result<Response, String> {
        self.client.fetch(url)
    }
}
