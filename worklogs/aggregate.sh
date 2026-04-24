#!/bin/bash
# aggregate.sh: Rebuild the worklog INDEX
# Wrapper for tooling/worklog-aggregator Rust binary
# (Bash justified: 3-line wrapper that sources PATH and execs compiled binary)

set -e
cd "$(git rev-parse --show-toplevel)" 2>/dev/null || cd "$(dirname "$(dirname "$0")")"
/Users/kooshapari/CodeProjects/Phenotype/repos/tooling/worklog-aggregator/target/release/worklog-aggregator
