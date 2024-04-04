# AWS Lambda Log Filter

[![version](https://img.shields.io/github/v/tag/DiscreteTom/aws-lambda-log-filter?label=release&style=flat-square)](https://github.com/DiscreteTom/aws-lambda-log-filter/releases/latest)
![license](https://img.shields.io/github/license/DiscreteTom/aws-lambda-log-filter?style=flat-square)
![rust](https://img.shields.io/badge/built_with-rust-DEA584?style=flat-square)

A lambda layer to save your money.

![log-flow](./img/log-flow.png)

> Powered by [AWS Lambda Log Proxy](https://github.com/DiscreteTom/aws-lambda-log-proxy).

## Usage

### As a Lambda Layer

1. Download the prebuilt zip from the [release page](https://github.com/DiscreteTom/aws-lambda-log-filter/releases/latest). You can also build it yourself by running `cargo build --release`, then zip `scripts/entry.sh` with `target/release/aws-lambda-log-filter`.
2. Upload the zip as a lambda layer. Add the layer to your lambda function.
3. Add an environment variable `AWS_LAMBDA_EXEC_WRAPPER` to the lambda function with the value `/opt/entry.sh` to enable the filter process.
4. Configure the [environment variables](#environment-variables) below to control how to filter the logs.

### As a Binary Executable

If you are using a custom lambda runtime or container image, you can run the filter as a parent process of your main handler process.

1. Download the prebuilt zip from the [release page](https://github.com/DiscreteTom/aws-lambda-log-filter/releases/latest) to get the `aws-lambda-log-filter` executable. You can also build it yourself by running `cargo build --release`.
2. Modify the entry command of the lambda function to `aws-lambda-log-filter <handler-command> <handler-args>`
3. Configure the [environment variables](#environment-variables) below to control how to filter the logs.

### Environment Variables

- `AWS_LAMBDA_LOG_FILTER_FILTER_BY_PREFIX`
  - If set, only log lines that start with this prefix will be kept.
- `AWS_LAMBDA_LOG_FILTER_IGNORE_BY_PREFIX`
  - If set, log lines that start with this prefix will be ignored.
- `AWS_LAMBDA_LOG_FILTER_FILTER_BY_REGEX`
  - If set, only log lines that match this regex will be kept.
  - The regex must be a valid [rust regex](https://docs.rs/regex/latest/regex/#syntax).
  - Keep the regex simple to avoid performance issues.
- `AWS_LAMBDA_LOG_FILTER_IGNORE_BY_REGEX`
  - If set, log lines that match this regex will be ignored.
  - The regex must be a valid [rust regex](https://docs.rs/regex/latest/regex/#syntax).
  - Keep the regex simple to avoid performance issues.
- `AWS_LAMBDA_LOG_FILTER_WRAP_IN_JSON_LEVEL`
  - If set, log lines will be wrapped in JSON with this value as the log level.
  - E.g. `INFO`, `ERROR`, `DEBUG`, etc.
- `AWS_LAMBDA_LOG_FILTER_DISABLE_LAMBDA_TELEMETRY_LOG_FD_FOR_HANDLER`
  - If set to `true`, the lambda telemetry log file descriptor will be disabled for the handler.
  - If you find the filter is not working (especially in nodejs), try to set this to `true`.

### Best Practices

- If you already enabled Lambda JSON format logging, or you are able to modify your code to use JSON format logging, you should avoid using this lambda layer. Please use the built-in JSON log-level filtering.
- Avoid to log JSON format logs in the lambda function, because the log filter will parse JSON format lines to check if the line is [EMF](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Embedded_Metric_Format_Specification.html) or not, which is costly. But you can set `AWS_LAMBDA_LOG_FILTER_WRAP_IN_JSON_LEVEL` in the environment variables to wrap the log lines in JSON format.
- If you want to use the built-in JSON log-level filtering with this filter simultaneously, after you enable the Lambda JSON format logging, you should set `AWS_LAMBDA_LOG_FORMAT` to `TEXT` in the environment variables to log in text format, then set `AWS_LAMBDA_LOG_FILTER_WRAP_IN_JSON_LEVEL` to the log level you want to filter.
- Since the logs are processed asynchronously, the timestamp generated by the filter process may have a small delay. To obtain the accurate log time, you might need to add a timestamp to the original log line.
- Increase the memory of your lambda function to obtain more CPU power to balance the cost of the lambda layer.

## FAQ

- Q: The filter configuration is not working, all logs are still there.
  - Try to set `AWS_LAMBDA_LOG_FILTER_DISABLE_LAMBDA_TELEMETRY_LOG_FD_FOR_HANDLER` to `true`.
- Q: How this works? / How to write my own version?
  - See [AWS Lambda Log Proxy](https://github.com/DiscreteTom/aws-lambda-log-proxy).

## [CHANGELOG](./CHANGELOG.md)
