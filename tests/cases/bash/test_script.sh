#!/bin/bash

# This is a basic shell script for testing purposes
echo "Hello, this is a test script."

# Initialize counter
counter=1

# While loop example
while [ $counter -le 5 ]
do
  echo "Counter: $counter"
  
  # If statement example
  if [ $counter -eq 3 ]
  then
    echo "Counter is equal to 3"
  fi
  
  # Increment counter
  counter=$((counter + 1))
done
