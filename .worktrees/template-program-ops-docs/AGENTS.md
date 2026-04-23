# Claude AI Agent Guide — template-program-ops

This repository is designed to work seamlessly with Claude (and other advanced AI agents) as an autonomous software engineer.

## Quick Start

```bash
# Install dependencies
uv sync

# Run CLI
uv run python -m ops_cli --help

# Test
uv run pytest

# Lint
uv run ruff check .
uv run ruff format .
```

## Repository Mental Model

### Project Structure

```
src/
  ops/
    __init__.py
    cli.py               # Typer CLI app
    commands/
      __init__.py
      deploy.py
      status.py
    config.py            # Pydantic config
    logging.py           # structlog setup
    tasks/
      __init__.py
      base.py            # Task base class
      scheduler.py       # Task scheduler
tests/
  conftest.py
  test_cli.py
  test_tasks.py
pyproject.toml
```

### Style Constraints

- **Line length:** 100 characters
- **Formatter:** Ruff
- **Linter:** Ruff with strict rules
- **Type checker:** pyright strict
- **Async:** Use asyncio for I/O

### Agent Must

- Use Typer for CLI apps
- Rich for formatted output
- structlog for logging
- Pydantic for config
- pytest for testing

## Standard Operating Loop

1. **Review** - Read requirements
2. **Research** - Check patterns
3. **Plan** - Design CLI interface
4. **Execute** - Implement commands
5. **Test** - pytest verification
6. **Polish** - Format and lint

## CLI Reference

```bash
# Main commands
uv run python -m ops_cli --help
uv run python -m ops_cli deploy --env prod
uv run python -m ops_cli status --verbose

# Development
uv run python -m ops_cli dev --reload
uv run python -m ops_cli shell

# Testing
uv run pytest
uv run pytest tests/unit
uv run pytest --cov=src
```

## Architecture Patterns

### Typer CLI

```python
import typer
from rich.console import Console
from typing import Optional

app = typer.Typer(help="Operations CLI")
console = Console()

@app.command()
def deploy(
    environment: str = typer.Option(..., help="Target environment"),
    dry_run: bool = typer.Option(False, "--dry-run", help="Simulate only"),
    verbose: bool = typer.Option(False, "--verbose", "-v"),
):
    """Deploy to target environment."""
    if verbose:
        console.print(f"[blue]Deploying to {environment}...[/blue]")
    
    if dry_run:
        console.print("[yellow]DRY RUN - no changes made[/yellow]")
        return
    
    # Deploy logic here
    console.print(f"[green]Deployed to {environment}[/green]")

@app.command()
def status(
    detailed: bool = typer.Option(False, "--detailed"),
):
    """Show deployment status."""
    # Status logic
    console.print("[green]All systems operational[/green]")
```

### Structured Logging

```python
import structlog
from structlog.types import Processor

def configure_logging(verbose: bool = False) -> None:
    processors: list[Processor] = [
        structlog.contextvars.merge_contextvars,
        structlog.stdlib.add_log_level,
        structlog.stdlib.add_logger_name,
        structlog.processors.TimeStamper(fmt="iso"),
        structlog.processors.StackInfoRenderer(),
        structlog.processors.format_exc_info,
    ]
    
    if verbose:
        processors.append(structlog.dev.ConsoleRenderer())
    else:
        processors.append(structlog.processors.JSONRenderer())
    
    structlog.configure(
        processors=processors,
        wrapper_class=structlog.stdlib.BoundLogger,
        context_class=dict,
        logger_factory=structlog.PrintLoggerFactory(),
        cache_logger_on_first_use=True,
    )
```

### Pydantic Config

```python
from pydantic import BaseModel, Field
from pathlib import Path
from typing import Optional
import tomllib

class DatabaseConfig(BaseModel):
    host: str = "localhost"
    port: int = 5432
    name: str
    user: str
    password: str = Field(..., env="DB_PASSWORD")

class AppConfig(BaseModel):
    debug: bool = False
    log_level: str = "INFO"
    database: DatabaseConfig

    @classmethod
    def from_toml(cls, path: Path) -> "AppConfig":
        with open(path, "rb") as f:
            data = tomllib.load(f)
        return cls.model_validate(data)
```

### Task Scheduling

```python
import asyncio
from abc import ABC, abstractmethod
from typing import Any
from datetime import datetime, timedelta

class Task(ABC):
    name: str
    
    @abstractmethod
    async def execute(self) -> Any:
        """Execute the task."""
        pass

class Scheduler:
    def __init__(self):
        self._tasks: list[tuple[Task, timedelta]] = []
    
    def schedule(self, task: Task, interval: timedelta) -> None:
        self._tasks.append((task, interval))
    
    async def run(self) -> None:
        while True:
            for task, interval in self._tasks:
                try:
                    result = await task.execute()
                    # Log result
                except Exception as e:
                    # Handle error
                    pass
            await asyncio.sleep(60)
```

## Testing Patterns

```python
import pytest
from typer.testing import CliRunner
from unittest.mock import patch

@pytest.fixture
def cli_runner():
    return CliRunner()

@pytest.fixture
def mock_deploy():
    with patch("ops.commands.deploy.deploy_service") as mock:
        mock.return_value = {"status": "success"}
        yield mock

def test_deploy_command(cli_runner, mock_deploy):
    result = cli_runner.invoke(app, ["deploy", "--env", "staging"])
    assert result.exit_code == 0
    assert "Deployed" in result.stdout
    mock_deploy.assert_called_once_with("staging", dry_run=False)
```

## Security Guidelines

- Never log secrets
- Environment variables for sensitive data
- Validate all input
- Rate limit commands
- Audit trail for operations
- Secure defaults

## Troubleshooting

```bash
# Dependency issues
uv lock --refresh
uv sync

# CLI issues
uv run python -m ops_cli --help
uv run python -m ops_cli --verbose deploy --env prod

# Test issues
uv run pytest -v --tb=long
```
