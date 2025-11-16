pub mod captcha_task;
pub mod turnstile_result;

#[derive(Debug, Clone, Copy)]
pub enum Status {
    Processing,
    Ready,
}

#[derive(Debug, Clone, Copy)]
pub enum CaptchaTask {
    Turnstile,
}
