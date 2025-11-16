use crate::setup::setup_api;
use capsolver::CaptchaTask::Turnstile;
mod setup;

#[tokio::test]
async fn test_create_task() {
    let solver = setup_api();

    solver.create_task(Turnstile).await;
}
