# CHANGELOG

## v0.2.0

- **_Breaking Change_**: `_LAMBDA_TELEMETRY_LOG_FD` is disabled for the handler process by default.
- Feat: add environment variable `AWS_LAMBDA_LOG_FILTER_SINK`.
- Perf: fast fail if the log line doesn't ends with `}` when checking EMF.
- Perf: apply single thread tokio executor.
- Perf: apply aws-lambda-log-proxy@0.2.0.

## v0.1.2

- Note: bump dependencies to avoid h2 vulnerability. See https://seanmonstar.com/blog/hyper-http2-continuation-flood/.

## v0.1.1

- Fix: ensure logs are processed before the lambda execution environment is freezed.
- Feat: use the `lambda_telemetry_log_fd` sink by default, fallback to `stdout`.

## v0.1.0

The initial release.
