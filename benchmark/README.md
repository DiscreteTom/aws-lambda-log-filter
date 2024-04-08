# Benchmark

## Steps

```bash
# go to the root folder of this project
cd ..
RUSTFLAGS="-C link-arg=-s" cargo build --release --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/aws-lambda-log-filter layer
cp scripts/entry.sh layer

cd benchmark
sam build
sam deploy -g
```

> In one line:
>
> ```bash
> cd .. && RUSTFLAGS="-C link-arg=-s" cargo build --release --target x86_64-unknown-linux-musl && cp target/x86_64-unknown-linux-musl/release/aws-lambda-log-filter layer && cd benchmark && sam build && sam deploy
> ```

The SAM will deploy the stack with an API. Test it with `plow`:

```bash
# e.g. disable the layer, test 1000 requests, 10 log messages per invocation
plow -n 1000 https://abcdefgh.execute-api.us-east-1.amazonaws.com/Prod/disabled/10
# e.g. enable the layer, test 1000 requests, 10 log messages per invocation
plow -n 1000 https://abcdefgh.execute-api.us-east-1.amazonaws.com/Prod/enabled/10
```

After tests with `plow`, checkout integration latency in API Gateway's metrics and duration in lambda metrics.
