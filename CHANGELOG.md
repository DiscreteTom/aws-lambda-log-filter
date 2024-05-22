# CHANGELOG

## v0.3.0

- **_Breaking Change_**: the filter will read from `stdin` and won't spawn the handler process.
  - Users should use wrapper scripts to redirect the output of the handler process to the proxy process using pipes (`|`).
  - This is to use output redirection (`2>&1`) provided by the system to ensure the log order across `stdout` and `stderr` is correct.
- **_Breaking Change_**: remove `AWS_LAMBDA_LOG_FILTER_DISABLE_LAMBDA_TELEMETRY_LOG_FD_FOR_HANDLER` environment variable.

## v0.2.1

- Feat: add environment variable `AWS_LAMBDA_LOG_FILTER_PROXY_BUFFER_SIZE` and `AWS_LAMBDA_LOG_FILTER_SINK_BUFFER_SIZE`.

## v0.2.0

- **_Breaking Change_**: `_LAMBDA_TELEMETRY_LOG_FD` is disabled for the handler process by default.
- Feat: add environment variable `AWS_LAMBDA_LOG_FILTER_SINK`.
- Perf: fast fail if the log line doesn't ends with `}` when checking EMF.
- Perf: apply single thread tokio executor.
- Perf: apply aws-lambda-log-proxy@0.2.

## v0.1.2

- Note: bump dependencies to avoid h2 vulnerability. See https://seanmonstar.com/blog/hyper-http2-continuation-flood/.

## v0.1.1

- Fix: ensure logs are processed before the lambda execution environment is freezed.
- Feat: use the `lambda_telemetry_log_fd` sink by default, fallback to `stdout`.

## v0.1.0

The initial release.
