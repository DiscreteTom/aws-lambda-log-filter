mod transformer;

use aws_lambda_log_proxy::{LogProxy, Sink};
use transformer::TransformerFactory;

fn create_proxy() -> LogProxy {
  let sink = Sink::lambda_telemetry_log_fd().unwrap_or_else(|_| Sink::stdout());
  let tf = TransformerFactory::new();

  LogProxy::default()
    .disable_lambda_telemetry_log_fd(
      std::env::var("AWS_LAMBDA_LOG_FILTER_DISABLE_LAMBDA_TELEMETRY_LOG_FD_FOR_HANDLER")
        .map(|s| s == "true")
        .unwrap_or(false),
    )
    .stdout(|p| p.transformer(tf.create()).sink(sink.clone()))
    .stderr(|p| p.transformer(tf.create()).sink(sink))
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  create_proxy().start().await;
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::env;

  #[test]
  fn test_create_proxy() {
    let proxy = create_proxy();
    assert_eq!(proxy.disable_lambda_telemetry_log_fd, false);
    assert_eq!(proxy.stdout.is_some(), true);
    assert_eq!(proxy.stderr.is_some(), true);

    env::set_var(
      "AWS_LAMBDA_LOG_FILTER_DISABLE_LAMBDA_TELEMETRY_LOG_FD_FOR_HANDLER",
      "true",
    );
    let proxy = create_proxy();
    assert_eq!(proxy.disable_lambda_telemetry_log_fd, true);
    env::remove_var("AWS_LAMBDA_LOG_FILTER_DISABLE_LAMBDA_TELEMETRY_LOG_FD_FOR_HANDLER");
  }
}
