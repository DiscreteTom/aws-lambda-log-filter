mod transformer;

use aws_lambda_log_proxy::{LogProxy, SimpleProcessor, Sink, SinkHandle};
use tokio::io::AsyncWrite;
use transformer::TransformerFactory;

#[tokio::main]
async fn main() {
  create_proxy().start().await;
}

fn create_proxy() -> LogProxy<SimpleProcessor, SimpleProcessor> {
  let sink = std::env::var("AWS_LAMBDA_LOG_FILTER_SINK")
    .map(|s| match s.as_str() {
      "stdout" => prepare_sink(Sink::stdout()),
      "stderr" => prepare_sink(Sink::stderr()),
      "telemetry_log_fd" => prepare_sink(Sink::lambda_telemetry_log_fd().unwrap()),
      _ => panic!("Invalid sink: {s}"),
    })
    .unwrap_or_else(|_| {
      // user doesn't specify the sink, prefer telemetry_log_fd if available, otherwise stdout
      Sink::lambda_telemetry_log_fd()
        .map(|s| prepare_sink(s))
        .unwrap_or_else(|_| prepare_sink(Sink::stdout()))
    });

  let tf = TransformerFactory::new();

  let mut proxy = LogProxy::new()
    .disable_lambda_telemetry_log_fd_for_handler(
      std::env::var("AWS_LAMBDA_LOG_FILTER_DISABLE_LAMBDA_TELEMETRY_LOG_FD_FOR_HANDLER")
        .map(|s| s == "true")
        .unwrap_or(true),
    )
    .stdout(|p| p.transformer(tf.create()).sink(sink.clone()))
    .stderr(|p| p.transformer(tf.create()).sink(sink));

  if let Some(size) = parse_buffer_size("AWS_LAMBDA_LOG_FILTER_PROXY_BUFFER_SIZE") {
    proxy = proxy.buffer_size(size)
  }

  proxy
}

fn prepare_sink<T: AsyncWrite + Send + Unpin + 'static>(mut sink: Sink<T>) -> SinkHandle {
  if let Some(size) = parse_buffer_size("AWS_LAMBDA_LOG_FILTER_SINK_BUFFER_SIZE") {
    sink = sink.buffer_size(size)
  }

  sink.spawn()
}

fn parse_buffer_size(env: &str) -> Option<usize> {
  std::env::var(env).ok().map(|s| {
    s.parse::<usize>()
      .unwrap_or_else(|_| panic!("Invalid buffer size: {s}"))
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::env;

  #[tokio::test]
  async fn test_create_proxy_default() {
    env::remove_var("AWS_LAMBDA_LOG_FILTER_SINK");
    let proxy = create_proxy();
    assert_eq!(proxy.disable_lambda_telemetry_log_fd_for_handler, true);
    assert_eq!(proxy.stdout.is_some(), true);
    assert_eq!(proxy.stderr.is_some(), true);
  }

  #[tokio::test]
  async fn test_enable_lambda_telemetry_log_fd_for_handler() {
    env::set_var(
      "AWS_LAMBDA_LOG_FILTER_DISABLE_LAMBDA_TELEMETRY_LOG_FD_FOR_HANDLER",
      "0",
    );
    let proxy = create_proxy();
    assert_eq!(proxy.disable_lambda_telemetry_log_fd_for_handler, false);
    env::remove_var("AWS_LAMBDA_LOG_FILTER_DISABLE_LAMBDA_TELEMETRY_LOG_FD_FOR_HANDLER");
  }

  #[tokio::test]
  async fn test_sink_stdout_stderr() {
    env::set_var("AWS_LAMBDA_LOG_FILTER_SINK", "stdout");
    create_proxy();
    env::remove_var("AWS_LAMBDA_LOG_FILTER_SINK");

    env::set_var("AWS_LAMBDA_LOG_FILTER_SINK", "stderr");
    create_proxy();
    env::remove_var("AWS_LAMBDA_LOG_FILTER_SINK");
  }

  #[tokio::test]
  async fn test_telemetry_log_fd_not_set() {
    env::set_var("AWS_LAMBDA_LOG_FILTER_SINK", "telemetry_log_fd");
    assert!(std::panic::catch_unwind(|| create_proxy()).is_err());
    env::remove_var("AWS_LAMBDA_LOG_FILTER_SINK");
  }

  #[tokio::test]
  async fn test_telemetry_log_fd() {
    env::set_var("AWS_LAMBDA_LOG_FILTER_SINK", "telemetry_log_fd");
    env::set_var("_LAMBDA_TELEMETRY_LOG_FD", "2");
    create_proxy();
    env::remove_var("_LAMBDA_TELEMETRY_LOG_FD");
    env::remove_var("AWS_LAMBDA_LOG_FILTER_SINK");
  }

  #[tokio::test]
  async fn test_invalid_sink() {
    env::set_var("AWS_LAMBDA_LOG_FILTER_SINK", "invalid");
    assert!(std::panic::catch_unwind(|| create_proxy()).is_err());
    env::remove_var("AWS_LAMBDA_LOG_FILTER_SINK");
  }
}
