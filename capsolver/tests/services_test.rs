use crate::setup::setup_services;
use capsolver::CaptchaTask::Turnstile;
use tokio::time::Instant;
mod setup;

#[tokio::test]
async fn test_solved() {
    let solver = setup_services();

    let now = Instant::now();
    let token = solver.solved(Turnstile).await;

    println!("{}", token);
    let s = now.elapsed();
    println!("{:?}", s)
}
