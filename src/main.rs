use aws_lambda_log_proxy::{LogProxy, Sink};
use serde_json::Value;

#[derive(Clone)]
struct Config {}

impl Config {
  pub fn new() -> Self {
    Self {}
  }

  pub fn create_transformer(&self) -> impl FnMut(String) -> Option<String> {
    let s = self.clone();
    move |line| s.transform(line)
  }

  fn transform(&self, line: String) -> Option<String> {
    if is_emf(&line) {
      // don't do any thing with emf logs
      return Some(line);
    }

    Some(line)
  }
}

#[tokio::main]
async fn main() {
  let sink = Sink::stdout();
  let config = Config::new();

  LogProxy::default()
    .disable_lambda_telemetry_log_fd(
      std::env::var("AWS_LAMBDA_LOG_FILTER_DISABLE_LAMBDA_TELEMETRY_LOG_FD")
        .map(|s| s == "true")
        .unwrap_or(false),
    )
    .stdout(|p| {
      p.transformer(config.create_transformer())
        .sink(sink.clone())
    })
    .stderr(|p| p.transformer(config.create_transformer()).sink(sink))
    .start()
    .await;
}

/// Return if the line is a valid JSON with the "_aws" key.
fn is_emf(line: &str) -> bool {
  serde_json::from_str(line)
    .ok()
    .map(|value: Value| value.get("_aws").is_some())
    .unwrap_or(false)
}
