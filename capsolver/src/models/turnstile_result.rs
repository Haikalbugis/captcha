use crate::{CaptchaTask, Status};
use anyhow::{Result, anyhow};
use serde_json::Value;

pub struct TurnstileResult {
    pub status: Status,
    pub token: Option<String>,
}

impl CaptchaTask {
    pub fn parsed(&self, res: &Value) -> Result<TurnstileResult> {
        match self {
            CaptchaTask::Turnstile => {
                if res["errorId"].as_i64() == Some(0) {
                    let status = res["status"].as_str();

                    if status == Some("processing") {
                        return Ok(TurnstileResult {
                            status: Status::Processing,
                            token: None,
                        });
                    } else if status == Some("ready") {
                        let token = res["solution"]["token"].as_str().map(|s| s.to_string());

                        return Ok(TurnstileResult {
                            status: Status::Ready,
                            token: token,
                        });
                    }
                }

                Err(anyhow!("Error parsed"))
            }
        }
    }
}
