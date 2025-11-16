use crate::CapsolverApi;

mod solved;

pub struct Capsolver {
    capsolver_api: CapsolverApi,
}

impl Capsolver {
    pub fn new(base_url: &str, client_key: &str, website_url: &str, website_key: &str) -> Self {
        let capsolver_api = CapsolverApi::new(base_url, client_key, website_url, website_key);

        Self { capsolver_api }
    }
}
