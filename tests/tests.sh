#!/usr/bin/env sh

# Hurl Vars
export HURL_url="http://localhost:8000"

# Run All Application Tests
# Ideally, they don't need to be executed in a particular sequence, but when things break this makes it a lot easier...
tests=(
    "tests/users.hurl"
    "tests/invites.hurl"
    "tests/permissions.hurl"
    "tests/sessions.hurl"
)
for test in "${tests[@]}"; do
    hurl --very-verbose "${test}"
done
