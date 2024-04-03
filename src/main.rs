mod transformer;

use aws_lambda_log_proxy::{LogProxy, Sink};
use transformer::TransformerFactory;

#[tokio::main]
async fn main() {
  let sink = Sink::stdout();
  let tf = TransformerFactory::new();

  LogProxy::default()
    .disable_lambda_telemetry_log_fd(
      std::env::var("AWS_LAMBDA_LOG_FILTER_DISABLE_LAMBDA_TELEMETRY_LOG_FD_FOR_HANDLER")
        .map(|s| s == "true")
        .unwrap_or(false),
    )
    .stdout(|p| p.transformer(tf.create()).sink(sink.clone()))
    .stderr(|p| p.transformer(tf.create()).sink(sink))
    .start()
    .await;
}
