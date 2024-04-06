# CHANGELOG

## v0.1.2

- Note: bump dependencies to avoid h2 vulnerability. See https://seanmonstar.com/blog/hyper-http2-continuation-flood/.

## v0.1.1

- Fix: ensure logs are processed before the lambda execution environment is freezed.
- Feat: use the `lambda_telemetry_log_fd` sink by default, fallback to `stdout`.

## v0.1.0

The initial release.
