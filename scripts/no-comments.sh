#!/bin/bash

set -e

found_comments=0

for file in $(find src tests -name "*.rs" 2>/dev/null); do
  if grep -n '^\s*//\|\s//\s\|/\*' "$file"; then
    found_comments=1
  fi
done

if [ $found_comments -eq 1 ]; then
  exit 1
fi

exit 0
