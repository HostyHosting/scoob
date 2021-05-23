#!/bin/sh

if [ "$TEST_KEY" = "another encrypted key" ]; then
  exit 0
else
  echo "Encrypted strings are not equal."
	exit 1
fi

