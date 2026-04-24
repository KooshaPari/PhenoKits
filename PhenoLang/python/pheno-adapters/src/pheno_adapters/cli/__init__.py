"""CLI Adapter for Pheno SDK.

This adapter translates CLI commands into use case calls following hexagonal
architecture. It handles input validation, output formatting, and error handling.
"""

from .adapter import CLIAdapter
from .commands import (
    ConfigurationCommands,
    DeploymentCommands,
    ServiceCommands,
    UserCommands,
)

__all__ = [
    "CLIAdapter",
    "ConfigurationCommands",
    "DeploymentCommands",
    "ServiceCommands",
    "UserCommands",
]
