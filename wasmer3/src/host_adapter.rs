use reqwest::Method;

#[async_trait::async_trait]
pub trait HostAdapter: Clone + Send + Sync + Unpin + 'static {
    async fn http_fetch(&self, url: String) -> Result<String, anyhow::Error>;
}

#[derive(Clone)]
pub struct VmAdapter {}

#[async_trait::async_trait]
impl HostAdapter for VmAdapter {
    async fn http_fetch(&self, url: String) -> Result<String, anyhow::Error> {
        let client = reqwest::Client::new();
        let response = client.request(Method::GET, url).send().await?;

        dbg!(&response);
        let bytes = response.bytes().await?;

        Ok(String::from_utf8_lossy(&bytes).to_string())
    }
}
