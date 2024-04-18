import { faker } from "@faker-js/faker";
import { Metrics, MetricUnit } from "@aws-lambda-powertools/metrics";

const LOG_COUNT = Number(process.env.LOG_COUNT);
const ADD_CPU_TASK = process.env.ADD_CPU_TASK == "true";

const metrics = new Metrics({
  namespace: "LambdaLogFilter",
  serviceName: "benchmark",
});

export const handler = async () => {
  let cpuTime = 0;
  for (let i = 0; i < LOG_COUNT; ++i) {
    // run some cpu bound task to simulate real life lambda tasks
    if (ADD_CPU_TASK) {
      const start = performance.now();
      calculatePrimes(1000);
      cpuTime += performance.now() - start;
    }

    // use process.stdout to avoid prepending other info
    process.stdout.write(faker.git.commitEntry() + "\n");
  }

  if (ADD_CPU_TASK) {
    process.stdout.write(`total cpu time: ${cpuTime}ms\n`);
  }

  metrics.addMetric("logs", MetricUnit.Count, LOG_COUNT);
  metrics.publishStoredMetrics();

  process.stdout.write(`done\n`);

  return {
    statusCode: 200,
    body: "hello",
  };
};

function calculatePrimes(n) {
  let primes = [];
  for (let i = 2; i <= n; i++) {
    if (isPrime(i)) {
      primes.push(i);
    }
  }
  return primes;
}

function isPrime(num) {
  for (let i = 2, sqrt = Math.sqrt(num); i <= sqrt; i++) {
    if (num % i === 0) {
      return false;
    }
  }
  return num > 1;
}
