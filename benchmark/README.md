# Benchmark

## Deploy

```bash
# run in the root folder of this project
RUSTFLAGS="-C link-arg=-s" cargo build --release --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/aws-lambda-log-filter layer
cp scripts/entry.sh layer

cd benchmark
sam build
sam deploy # maybe add '-g' for the first time
cd ..
```

In one line:

```bash
RUSTFLAGS="-C link-arg=-s" cargo build --release --target x86_64-unknown-linux-musl && cp target/x86_64-unknown-linux-musl/release/aws-lambda-log-filter layer && cp scripts/entry.sh layer && cd benchmark && sam build && sam deploy && cd ..
```

## Validate

Run [`./validate.sh`](./validate.sh) to ensure the `invocation/next` is properly suppressed.

## Test

The SAM will deploy the stack with an API. Test it with `plow`:

```bash
# e.g. disable the layer, test 1000 requests, 10 log messages (~4 lines per message) per invocation
plow -n 1000 https://abcdefgh.execute-api.us-east-1.amazonaws.com/Prod/10/disabled
# e.g. enable the layer, test 1000 requests, 10 log messages (~4 lines per message) per invocation
plow -n 1000 https://abcdefgh.execute-api.us-east-1.amazonaws.com/Prod/10/enabled
```

After tests with `plow`, checkout integration latency in API Gateway's metrics and duration in lambda metrics.

Or with [Lambda Power Tuning](https://github.com/alexcasalboni/aws-lambda-power-tuning).
