use crate::{CaptchaTask, services::Capsolver};
use std::time::Duration;
use tokio::time::sleep;

impl Capsolver {
    pub async fn solved(&self, captcha_task: CaptchaTask) -> String {
        let task_id = match self.capsolver_api.create_task(captcha_task).await {
            Ok(task_id) => task_id,
            Err(err) => {
                panic!("Error task id: {}", err.to_string());
            }
        };

        loop {
            let result = self
                .capsolver_api
                .get_task_result(&task_id, captcha_task)
                .await;

            match result {
                Ok(status) => match status.status {
                    crate::Status::Processing => {}
                    crate::Status::Ready => return status.token.unwrap(),
                },
                Err(err) => panic!("Error get task result"),
            }

            sleep(Duration::from_millis(100)).await;
        }
    }
}
