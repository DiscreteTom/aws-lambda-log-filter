use aws_lambda_log_proxy::{LogProxy, Sink};
use regex::Regex;
use serde_json::Value;

#[derive(Clone)]
struct TransformerFactory {
  filter_by_prefix: Option<String>,
  ignore_by_prefix: Option<String>,
  filter_by_regex: Option<Regex>,
  ignore_by_regex: Option<Regex>,
  wrap_in_json_level: Option<String>,
}

impl TransformerFactory {
  pub fn new() -> Self {
    Self {
      filter_by_prefix: std::env::var("AWS_LAMBDA_LOG_FILTER_FILTER_BY_PREFIX").ok(),
      ignore_by_prefix: std::env::var("AWS_LAMBDA_LOG_FILTER_IGNORE_BY_PREFIX").ok(),
      filter_by_regex: std::env::var("AWS_LAMBDA_LOG_FILTER_FILTER_BY_REGEX")
        .ok()
        .map(|s| Regex::new(&s).unwrap()),
      ignore_by_regex: std::env::var("AWS_LAMBDA_LOG_FILTER_IGNORE_BY_REGEX")
        .ok()
        .map(|s| Regex::new(&s).unwrap()),
      wrap_in_json_level: std::env::var("AWS_LAMBDA_LOG_FILTER_WRAP_IN_JSON_LEVEL").ok(),
    }
  }

  /// Create a transformer that filters and transforms the log lines.
  pub fn create(&self) -> impl FnMut(String) -> Option<String> {
    let s = self.clone();
    move |line| s.transform(line)
  }

  fn transform(&self, mut line: String) -> Option<String> {
    if is_emf(&line) {
      // don't do any thing with emf logs
      return Some(line);
    }

    if let Some(prefix) = &self.filter_by_prefix {
      if !line.starts_with(prefix) {
        return None;
      }
    }

    if let Some(prefix) = &self.ignore_by_prefix {
      if line.starts_with(prefix) {
        return None;
      }
    }

    if let Some(re) = &self.filter_by_regex {
      if !re.is_match(&line) {
        return None;
      }
    }

    if let Some(re) = &self.ignore_by_regex {
      if re.is_match(&line) {
        return None;
      }
    }

    if let Some(level) = &self.wrap_in_json_level {
      line = serde_json::json!({
        "level": level,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "message": line,
      })
      .to_string();
    }

    Some(line)
  }
}

#[tokio::main]
async fn main() {
  let sink = Sink::stdout();
  let tf = TransformerFactory::new();

  LogProxy::default()
    .disable_lambda_telemetry_log_fd(
      std::env::var("AWS_LAMBDA_LOG_FILTER_DISABLE_LAMBDA_TELEMETRY_LOG_FD")
        .map(|s| s == "true")
        .unwrap_or(false),
    )
    .stdout(|p| p.transformer(tf.create()).sink(sink.clone()))
    .stderr(|p| p.transformer(tf.create()).sink(sink))
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
