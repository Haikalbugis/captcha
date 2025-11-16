use client::client::ApiClient;

mod create_task;
pub mod get_task_result;

pub struct CapsolverApi {
    client: ApiClient,
    pub client_key: String,
    pub website_url: String,
    pub website_key: String,
}

impl CapsolverApi {
    pub fn new(base_url: &str, client_key: &str, website_url: &str, website_key: &str) -> Self {
        let client = ApiClient::new(base_url.to_string()).unwrap();
        client.set_header("host", "api.capsolver.com");
        client.set_header("content-type", "application/json");

        Self {
            client,
            client_key: client_key.to_string(),
            website_url: website_url.to_string(),
            website_key: website_key.to_string(),
        }
    }
}
