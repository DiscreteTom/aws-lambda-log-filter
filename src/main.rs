mod transformer;

use aws_lambda_log_proxy::{LogProxy, SimpleProcessor, Sink};
use transformer::TransformerFactory;

fn create_proxy() -> LogProxy<SimpleProcessor, SimpleProcessor> {
  let sink = Sink::lambda_telemetry_log_fd()
    .map(|s| s.spawn())
    .unwrap_or_else(|_| Sink::stdout().spawn());
  let tf = TransformerFactory::new();

  LogProxy::new()
    .disable_lambda_telemetry_log_fd_for_handler(
      std::env::var("AWS_LAMBDA_LOG_FILTER_DISABLE_LAMBDA_TELEMETRY_LOG_FD_FOR_HANDLER")
        .map(|s| s == "true")
        .unwrap_or(false),
    )
    .stdout(|p| p.transformer(tf.create()).sink(sink.clone()))
    .stderr(|p| p.transformer(tf.create()).sink(sink))
}

fn main() {
  if std::env::var("AWS_LAMBDA_LOG_FILTER_MULTI_THREAD")
    .map(|s| s == "true")
    .unwrap_or(false)
  {
    tokio::runtime::Builder::new_multi_thread()
  } else {
    tokio::runtime::Builder::new_current_thread()
  }
  .enable_all()
  .build()
  .unwrap()
  .block_on(async {
    create_proxy().start().await;
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::env;

  #[test]
  fn test_create_proxy() {
    let proxy = create_proxy();
    assert_eq!(proxy.disable_lambda_telemetry_log_fd_for_handler, false);
    assert_eq!(proxy.stdout.is_some(), true);
    assert_eq!(proxy.stderr.is_some(), true);

    env::set_var(
      "AWS_LAMBDA_LOG_FILTER_DISABLE_LAMBDA_TELEMETRY_LOG_FD_FOR_HANDLER",
      "true",
    );
    let proxy = create_proxy();
    assert_eq!(proxy.disable_lambda_telemetry_log_fd_for_handler, true);
    env::remove_var("AWS_LAMBDA_LOG_FILTER_DISABLE_LAMBDA_TELEMETRY_LOG_FD_FOR_HANDLER");
  }
}
