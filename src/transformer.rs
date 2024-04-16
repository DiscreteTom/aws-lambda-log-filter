use chrono::Utc;
use regex::Regex;
use serde_json::{json, Value};

#[derive(Clone)]
pub(crate) struct TransformerFactory {
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
      line = json!({
        "timestamp": Utc::now().format("%Y-%m-%dT%H:%M:%S%.6fZ").to_string(),
        "level": level,
        "message": line,
      })
      .to_string();
    }

    Some(line)
  }
}

/// Return if the line is a valid JSON object with the `"_aws"` key.
fn is_emf(line: &str) -> bool {
  // perf: check if the line is wrapped with `{}` before parsing it as JSON
  // so we can fast fail if it's not a JSON object
  let trimmed = line.trim();
  if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
    return false;
  }

  serde_json::from_str(trimmed)
    .ok()
    .map(|value: Value| value.get("_aws").is_some())
    .unwrap_or(false)
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::{collections::HashMap, env};

  #[test]
  fn check_emf() {
    // compact
    assert!(is_emf(r#"{"_aws":{"key":"value"}}"#));
    // with whitespace
    assert!(is_emf(r#"{"_aws": {"key": "value"}}"#));
    assert!(is_emf(r#"  {  "_aws"  : {"key": "value"}  }  "#));

    // missing _aws key
    assert!(!is_emf(r#"{"key": "value"}"#));
    assert!(!is_emf(r#"{"  _aws":{"key":"value"}}"#));
    // invalid JSON
    assert!(!is_emf(r#"{"_aws": {"key": "value"}"#));
    // not a JSON object
    assert!(!is_emf("123"));
  }

  fn assert_kept(factory: &TransformerFactory, line: &str) {
    assert_eq!(factory.create()(line.to_string()), Some(line.to_string()));
  }
  fn assert_kept_emf(factory: &TransformerFactory) {
    // emf will always be kept and not modified
    let line = r#"{"_aws":{"key":"value"}}"#;
    assert_kept(factory, line);
  }
  fn assert_kept_json(factory: &TransformerFactory, line: &str, level: &str) {
    let res: Value = serde_json::from_str(&factory.create()(line.to_string()).unwrap()).unwrap();
    assert_eq!(res["level"], level);
    assert!(res["timestamp"].is_string());
    assert_eq!(
      res["timestamp"].as_str().unwrap().len(),
      "2023-11-02T16:51:31.587199Z".len()
    );
    assert_eq!(res["message"], line);
  }
  fn assert_ignored(factory: &TransformerFactory, line: &str) {
    assert_eq!(factory.create()(line.to_string()), None);
  }
  fn with_env(map: HashMap<&str, &str>, f: impl Fn()) {
    for (key, value) in &map {
      env::set_var(key, value);
    }
    f();
    for (key, _) in &map {
      env::remove_var(key);
    }
  }

  #[test]
  fn transformer_factory_new() {
    let tf = TransformerFactory::new();
    assert!(matches!(tf.filter_by_prefix, None));
    assert!(matches!(tf.ignore_by_prefix, None));
    assert!(matches!(tf.filter_by_regex, None));
    assert!(matches!(tf.ignore_by_regex, None));
    assert!(matches!(tf.wrap_in_json_level, None));

    with_env(
      HashMap::from([
        ("AWS_LAMBDA_LOG_FILTER_FILTER_BY_PREFIX", "filter-prefix"),
        ("AWS_LAMBDA_LOG_FILTER_IGNORE_BY_PREFIX", "ignore-prefix"),
        ("AWS_LAMBDA_LOG_FILTER_FILTER_BY_REGEX", r"^\d+$"),
        ("AWS_LAMBDA_LOG_FILTER_IGNORE_BY_REGEX", r"^\d+$"),
        ("AWS_LAMBDA_LOG_FILTER_WRAP_IN_JSON_LEVEL", "DEBUG"),
      ]),
      || {
        let tf = TransformerFactory::new();
        assert_eq!(tf.filter_by_prefix, Some("filter-prefix".to_string()));
        assert_eq!(tf.ignore_by_prefix, Some("ignore-prefix".to_string()));
        assert_eq!(tf.filter_by_regex.unwrap().as_str(), r"^\d+$");
        assert_eq!(tf.ignore_by_regex.unwrap().as_str(), r"^\d+$");
        assert_eq!(tf.wrap_in_json_level, Some("DEBUG".to_string()));
      },
    );
  }

  #[test]
  fn transformer_factory() {
    // kept all by default
    let tf = TransformerFactory {
      filter_by_prefix: None,
      ignore_by_prefix: None,
      filter_by_regex: None,
      ignore_by_regex: None,
      wrap_in_json_level: None,
    };
    assert_kept(&tf, "123");
    assert_kept_emf(&tf);

    // filter by prefix
    let tf = TransformerFactory {
      filter_by_prefix: Some("prefix".to_string()),
      ignore_by_prefix: None,
      filter_by_regex: None,
      ignore_by_regex: None,
      wrap_in_json_level: None,
    };
    assert_kept(&tf, "prefix123");
    assert_ignored(&tf, "123");
    assert_kept_emf(&tf);

    // ignore by prefix
    let tf = TransformerFactory {
      filter_by_prefix: None,
      ignore_by_prefix: Some("prefix".to_string()),
      filter_by_regex: None,
      ignore_by_regex: None,
      wrap_in_json_level: None,
    };
    assert_ignored(&tf, "prefix123");
    assert_kept(&tf, "123");
    assert_kept_emf(&tf);

    // filter by regex
    let tf = TransformerFactory {
      filter_by_prefix: None,
      ignore_by_prefix: None,
      filter_by_regex: Some(Regex::new(r"^\d+$").unwrap()),
      ignore_by_regex: None,
      wrap_in_json_level: None,
    };
    assert_kept(&tf, "123");
    assert_ignored(&tf, "abc");
    assert_kept_emf(&tf);

    // ignore by regex
    let tf = TransformerFactory {
      filter_by_prefix: None,
      ignore_by_prefix: None,
      filter_by_regex: None,
      ignore_by_regex: Some(Regex::new(r"^\d+$").unwrap()),
      wrap_in_json_level: None,
    };
    assert_ignored(&tf, "123");
    assert_kept(&tf, "abc");
    assert_kept_emf(&tf);

    // wrap in json level
    let tf = TransformerFactory {
      filter_by_prefix: None,
      ignore_by_prefix: None,
      filter_by_regex: None,
      ignore_by_regex: None,
      wrap_in_json_level: Some("debug".to_string()),
    };
    assert_kept_json(&tf, "123", "debug");
    assert_kept_emf(&tf);

    // filter and wrap
    let tf = TransformerFactory {
      filter_by_prefix: Some("prefix".to_string()),
      ignore_by_prefix: None,
      filter_by_regex: None,
      ignore_by_regex: None,
      wrap_in_json_level: Some("debug".to_string()),
    };
    assert_kept_json(&tf, "prefix123", "debug");
    assert_ignored(&tf, "123");
    assert_kept_emf(&tf);
  }
}
