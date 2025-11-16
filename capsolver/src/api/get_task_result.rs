use crate::{CapsolverApi, CaptchaTask};
use anyhow::{Result, anyhow};
use serde_json::json;

impl CapsolverApi {
    pub async fn get_task_result(
        &self,
        task_id: &str,
        captcha_task: CaptchaTask,
    ) -> Result<crate::turnstile_result::TurnstileResult> {
        let body = json!({
          "clientKey": self.client_key,
          "taskId": task_id
        });

        let response = self
            .client
            .post_async_json("getTaskResult", body.to_string())
            .await;

        match response {
            Ok(res) => captcha_task.parsed(&res),
            Err(err) => Err(anyhow!(
                "Error request get task result: {}",
                err.to_string()
            )),
        }
    }
}
