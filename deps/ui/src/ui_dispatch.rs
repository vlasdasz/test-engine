use anyhow::Result;
use tokio::spawn;

use crate::views::AlertErr;

pub fn on_back(task: impl Future<Output = Result<()>> + Send + 'static) {
    spawn(async move {
        task.await.alert_err();
    });
}
