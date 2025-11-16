use crate::{CaptchaTask, api::CapsolverApi};
use anyhow::{Result, anyhow};

impl CapsolverApi {
    pub async fn create_task(&self, captcha_task: CaptchaTask) -> Result<String> {
        let body = captcha_task.to_body(self);

        let resposne = self
            .client
            .post_async_json("createTask", body.to_string())
            .await;

        match resposne {
            Ok(res) => {
                if res["errorId"].as_i64() == Some(0) {
                    let task_id = res["taskId"].as_str().unwrap();

                    return Ok(task_id.to_string());
                }

                Err(anyhow!("Error get task id"))
            }
            Err(err) => Err(anyhow!("Error request task id: {}", err.to_string())),
        }
    }
}
