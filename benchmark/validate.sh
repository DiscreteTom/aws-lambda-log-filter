#!/bin/bash

params=(10 100 1000 10000 100000)
for param in "${params[@]}"
do
  for ((i=1; i<=10; i++))
  do
    # ensure 'done' is printed
    if sam remote invoke "Enabled${param}LogFilterTestFunction" --stack-name LogFilterBenchmark 2>&1 | tail -n 4 | grep -q "done"; then
      echo -n "."
    else
      echo "validation failed, running Enabled${param}LogFilterTestFunction"
      exit 1
    fi
  done
done


echo 'passed!'