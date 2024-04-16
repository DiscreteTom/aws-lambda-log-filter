mod transformer;

use aws_lambda_log_proxy::{LogProxy, SimpleProcessor, Sink};
use transformer::TransformerFactory;

fn create_proxy() -> LogProxy<SimpleProcessor, SimpleProcessor> {
  let sink = std::env::var("AWS_LAMBDA_LOG_FILTER_SINK")
    .map(|s| match s.as_str() {
      "stdout" => Sink::stdout().spawn(),
      "stderr" => Sink::stderr().spawn(),
      "telemetry_log_fd" => Sink::lambda_telemetry_log_fd().unwrap().spawn(),
      _ => panic!("Invalid sink: {s}"),
    })
    .unwrap_or_else(|_| {
      // user doesn't specify the sink, prefer telemetry_log_fd if available, otherwise stdout
      Sink::lambda_telemetry_log_fd()
        .map(|s| s.spawn())
        .unwrap_or_else(|_| Sink::stdout().spawn())
    });

  let tf = TransformerFactory::new();

  LogProxy::new()
    .disable_lambda_telemetry_log_fd_for_handler(
      std::env::var("AWS_LAMBDA_LOG_FILTER_DISABLE_LAMBDA_TELEMETRY_LOG_FD_FOR_HANDLER")
        .map(|s| s == "true")
        .unwrap_or(true),
    )
    .stdout(|p| p.transformer(tf.create()).sink(sink.clone()))
    .stderr(|p| p.transformer(tf.create()).sink(sink))
}

#[tokio::main]
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
