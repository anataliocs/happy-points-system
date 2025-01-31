use loco_rs::{bgworker::BackgroundWorker, testing::prelude::*};
use happy_points_svc::{
    app::App,
    workers::storage_worker::{Worker, WorkerArgs},
};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_run_storage_worker_worker() {
    let boot = boot_test::<App>().await.unwrap();

    // Execute the worker ensuring that it operates in 'ForegroundBlocking' mode, which prevents the addition of your worker to the background
    assert!(
        Worker::perform_later(&boot.app_context,WorkerArgs {})
            .await
            .is_ok()
    );
    // Include additional assert validations after the execution of the worker
}
