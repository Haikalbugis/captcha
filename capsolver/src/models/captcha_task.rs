use crate::{CaptchaTask, api::CapsolverApi};
use serde_json::{Value, json};

impl CaptchaTask {
    pub fn to_body(&self, solver: &CapsolverApi) -> Value {
        match self {
            CaptchaTask::Turnstile => json!({
              "clientKey": solver.client_key,
              "task": {
                "type": "AntiTurnstileTaskProxyLess",
                "websiteURL": solver.website_url,
                "websiteKey": solver.website_key,
              }
            }),
        }
    }
}
