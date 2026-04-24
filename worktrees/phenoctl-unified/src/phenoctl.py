"""phenoctl - Unified CLI for phenotype ecosystem."""
from __future__ import annotations

__version__ = "0.1.0"
__author__ = "Phenotype Team"

from pathlib import Path

# Plugin registry
PLUGIN_REGISTRY: dict[str, type[CLIPlugin]] = {}


def register_plugin(name: str, plugin_class: type[CLIPlugin]) -> None:
    """Register a CLI plugin."""
    PLUGIN_REGISTRY[name] = plugin_class


class CLIPlugin:
    """Base class for phenoctl plugins."""

    name: str = "base"
    description: str = ""

    def __init__(self, config: dict | None = None):
        self.config = config or {}

    def execute(self, args: list[str]) -> int:
        """Execute the plugin with given arguments."""
        raise NotImplementedError

    def register_commands(self, subparsers) -> None:
        """Register argparse subcommands."""
        pass


class PluginHost:
    """Host for CLI plugins with lifecycle management."""

    def __init__(self):
        self.plugins: dict[str, CLIPlugin] = {}
        self._initialized = False

    def load_plugin(self, name: str, config: dict | None = None) -> CLIPlugin:
        """Load a plugin by name."""
        if name not in PLUGIN_REGISTRY:
            raise ValueError(f"Unknown plugin: {name}")
        
        plugin = PLUGIN_REGISTRY[name](config)
        self.plugins[name] = plugin
        return plugin

    def unload_plugin(self, name: str) -> None:
        """Unload a plugin."""
        if name in self.plugins:
            del self.plugins[name]

    def execute_plugin(self, name: str, args: list[str]) -> int:
        """Execute a plugin with args."""
        if name not in self.plugins:
            raise ValueError(f"Plugin not loaded: {name}")
        return self.plugins[name].execute(args)

    def list_plugins(self) -> list[str]:
        """List available plugins."""
        return list(self.plugins.keys())


__all__ = [
    "CLIPlugin",
    "PluginHost",
    "register_plugin",
    "PLUGIN_REGISTRY",
]
