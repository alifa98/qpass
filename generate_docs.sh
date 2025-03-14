#!/bin/bash

# Ensure the `docs` directory exists
mkdir -p docs

# Capture general help
echo -e "# CLI Help\n" > docs/cli.md
cargo run -- --help >> docs/cli.md

# Extract all top-level commands
COMMANDS=$(cargo run -- --help | grep -E "^\s{2}[a-zA-Z0-9_-]+" | awk '{print $1}')

# Iterate over each command and generate help documentation
for cmd in $COMMANDS; do
    echo "Generating documentation for command: $cmd"
    echo -e "# $cmd Command\n" > docs/$cmd.md
    cargo run -- $cmd --help >> docs/$cmd.md
done

echo "CLI documentation has been generated in the docs/ folder!"
