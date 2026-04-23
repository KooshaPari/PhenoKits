# Planify - TypeScript/Node project management
# Native task runner (just)

# Default recipe
default: help

# Help
help:
  @echo "Planify - TypeScript/Node project management"
  @echo ""
  @just --list

# Install dependencies
install:
  npm install

# Quality checks
check: lint typecheck test
  @echo "All checks passed!"

# Lint
lint:
  npm run lint

# Type check
typecheck:
  npm run typecheck

# Run tests
test:
  npm run test

# Build
build:
  npm run build

# Development
dev:
  npm run dev

# Database migrations
db:migrate:
  npm run db:migrate

db:seed:
  npm run db:seed

# Clean
clean:
  rm -rf dist node_modules/.cache .next
