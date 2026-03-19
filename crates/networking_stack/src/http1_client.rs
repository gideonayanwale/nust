#[derive(Debug, Clone)]
pub struct Response {
    pub status_code: u16,
    pub body: String,
}

pub trait HttpClient: Send + Sync {
    fn fetch(&self, url: &str) -> Result<Response, String>;
}

#[derive(Debug, Default)]
pub struct NativeHttp1Client;

impl HttpClient for NativeHttp1Client {
    fn fetch(&self, url: &str) -> Result<Response, String> {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(format!("invalid url: {url}"));
        }

        let body = format!(
            "<html><body><h1>NUST placeholder fetch</h1><p>Fetched URL: {}</p></body></html>",
            url
        );

        Ok(Response {
            status_code: 200,
            body,
        })
    }
}
