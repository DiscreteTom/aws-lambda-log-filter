# AWS Lambda Log Filter

[![Codecov](https://img.shields.io/codecov/c/github/DiscreteTom/aws-lambda-log-filter?style=flat-square)
](https://codecov.io/gh/DiscreteTom/aws-lambda-log-filter)
[![version](https://img.shields.io/github/v/tag/DiscreteTom/aws-lambda-log-filter?label=release&style=flat-square)](https://github.com/DiscreteTom/aws-lambda-log-filter/releases/latest)
![license](https://img.shields.io/github/license/DiscreteTom/aws-lambda-log-filter?style=flat-square)
![rust](https://img.shields.io/badge/built_with-rust-DEA584?style=flat-square)

A lambda layer to save your money.

![log-flow](./img/log-flow.png)

> [!CAUTION]
> Possible data loss if you write tons of logs and return immediately. See [possible data loss](#possible-data-loss) below.

## Usage

### As a Lambda Layer

1. Download the prebuilt zip from the [release page](https://github.com/DiscreteTom/aws-lambda-log-filter/releases/latest). You can also build it yourself by running `cargo build --release`, then zip `scripts/entry.sh` with `target/release/aws-lambda-log-filter`.
2. Upload the zip as a lambda layer. Add the layer to your lambda function.
3. Add an environment variable `AWS_LAMBDA_EXEC_WRAPPER` to the lambda function with the value `/opt/entry.sh` to enable the filter process.
4. Configure the [environment variables](#environment-variables) below to control how to filter the logs.

### As a Binary Executable

If you are using a custom lambda runtime (for rust, golang, c++, etc) or container image, you can run the filter as a parent process of your main handler process.

1. Download the prebuilt zip from the [release page](https://github.com/DiscreteTom/aws-lambda-log-filter/releases/latest) to get the `aws-lambda-log-filter` executable. You can also build it yourself by running `cargo build --release`.
2. Modify the entry command of the lambda function to `aws-lambda-log-filter <handler-command> <handler-args>`
3. Configure the [environment variables](#environment-variables) below to control how to filter the logs.

### Environment Variables

#### Filter Configuration

> [!NOTE]
>
> [EMF](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Embedded_Metric_Format_Specification.html) won't be affected by these environment variables.

- `AWS_LAMBDA_LOG_FILTER_FILTER_BY_PREFIX`
  - If set, only lines that start with this prefix will be kept.
- `AWS_LAMBDA_LOG_FILTER_IGNORE_BY_PREFIX`
  - If set, lines that start with this prefix will be ignored.
  - [EMF](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Embedded_Metric_Format_Specification.html) won't be affected.
- `AWS_LAMBDA_LOG_FILTER_FILTER_BY_REGEX`
  - If set, only lines that match this regex will be kept.
  - The regex must be a valid [rust regex](https://docs.rs/regex/latest/regex/#syntax).
  - Keep the regex simple to avoid performance issues.
  - [EMF](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Embedded_Metric_Format_Specification.html) won't be affected.
- `AWS_LAMBDA_LOG_FILTER_IGNORE_BY_REGEX`
  - If set, lines that match this regex will be ignored.
  - The regex must be a valid [rust regex](https://docs.rs/regex/latest/regex/#syntax).
  - Keep the regex simple to avoid performance issues.
  - [EMF](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Embedded_Metric_Format_Specification.html) won't be affected.

#### Output Enhancement

> [!NOTE]
>
> [EMF](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Embedded_Metric_Format_Specification.html) won't be affected by these environment variables.

- `AWS_LAMBDA_LOG_FILTER_WRAP_IN_JSON_LEVEL`
  - If set, non-ignored lines will be wrapped in JSON with this value as the log level.
  - E.g. `INFO`, `ERROR`, `DEBUG`, etc.

#### Misc

- `AWS_LAMBDA_LOG_FILTER_DISABLE_LAMBDA_TELEMETRY_LOG_FD_FOR_HANDLER`
  - If set to `true`, the lambda telemetry log file descriptor will be disabled for the handler process.
  - Any other value will be treated as `false`.
  - If not set, treat as `true`.
- `AWS_LAMBDA_LOG_FILTER_SINK`
  - The sink to output the filtered logs.
  - Available values: `stdout`, `stderr`, `telemetry_log_fd`.
  - If not specified, prefer `telemetry_log_fd` if available, otherwise `stdout`.

## FAQ

- Q: The filter configuration is not working, all logs are still there.
  - Ensure the environment variable `AWS_LAMBDA_LOG_FILTER_DISABLE_LAMBDA_TELEMETRY_LOG_FD_FOR_HANDLER` is not set, or set to `true`.
  - Ensure the environment variable `AWS_LAMBDA_EXEC_WRAPPER` is set to `/opt/entry.sh`.
  - [Wrapper scripts](https://docs.aws.amazon.com/lambda/latest/dg/runtimes-modify.html#runtime-wrapper) are not supported on [OS-only runtimes](https://docs.aws.amazon.com/lambda/latest/dg/runtimes-provided.html) (the provided runtime family). So the layer approach won't work for functions in rust, golang, c++, etc. Try the [binary executable approach](#as-a-binary-executable) instead.
- Q: `/opt/aws-lambda-log-filter: /lib64/libc.so.6: version 'GLIBC_2.28' not found` error when invoke the lambda function.
  - Build the binary with `musl` target to include the C runtime: `cargo build --release --target=x86_64-unknown-linux-musl`.
- Q: How this works? / How to write my own version? / How to write logs to other places like ElasticSearch?
  - See [AWS Lambda Log Proxy](https://github.com/DiscreteTom/aws-lambda-log-proxy).

## Cost and Performance

Performance may vary depending on the log volume and the filter configuration (e.g. a complex regex filter may slow down the process). If there are only hundreds lines of logs, and the filter is not complex, the performance impact should be negligible (usually less than 5ms, and since the processing is asynchronously, synchronous invokers like API Gateway won't be affected).

The cost is mainly from the additional execution time. This also vary depending on the log volume and the filter configuration.

However, it is pretty easy to compare between enabling and disabling the filter, just use the environment variable `AWS_LAMBDA_EXEC_WRAPPER` as a switch, test the performance and cost in your own real workloads, and decide whether to use it.

## Possible Data Loss

Though we tried our best to suppress the [`invocation/next`](https://docs.aws.amazon.com/lambda/latest/dg/runtimes-api.html#runtimes-api-next) request to process logs as much as possible, if you write tons of logs (more than thousands of lines) and return immediately, there might be some logs not processed.

As a best practice, it is your responsibility to do a thorough benchmark test against your own use case to ensure the logs are processed as expected.

## [CHANGELOG](./CHANGELOG.md)
