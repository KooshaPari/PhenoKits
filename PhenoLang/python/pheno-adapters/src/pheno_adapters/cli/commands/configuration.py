"""
Configuration CLI commands.
"""

from __future__ import annotations

from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from pheno.adapters.cli.adapter import CLIAdapter

from pheno.application.dtos.configuration import (
    ConfigurationFilterDTO,
    CreateConfigurationDTO,
    UpdateConfigurationDTO,
)


class ConfigurationCommands:
    """
    Configuration management commands for CLI.
    """

    def __init__(self, adapter: CLIAdapter):
        self.adapter = adapter

    async def create(self, key: str, value: Any, description: str | None = None) -> None:
        """
        Create a new configuration.
        """
        try:
            dto = CreateConfigurationDTO(key=key, value=value, description=description)
            config = await self.adapter.create_configuration.execute(dto)
            self.adapter.print_success(f"Configuration created: {config.key}")
            self.adapter.print_info(f"Value: {config.value}")
        except Exception as e:
            self.adapter.print_error(f"Failed to create configuration: {e}")
            raise

    async def update(
        self, key: str, value: Any | None = None, description: str | None = None,
    ) -> None:
        """
        Update a configuration.
        """
        try:
            dto = UpdateConfigurationDTO(key=key, value=value, description=description)
            config = await self.adapter.update_configuration.execute(dto)
            self.adapter.print_success(f"Configuration updated: {config.key}")
        except Exception as e:
            self.adapter.print_error(f"Failed to update configuration: {e}")
            raise

    async def get(self, key: str) -> None:
        """
        Get configuration details.
        """
        try:
            config = await self.adapter.get_configuration.execute(key)
            self.adapter.print_table(
                title=f"Configuration: {config.key}",
                columns=["Field", "Value"],
                rows=[
                    ["Key", config.key],
                    ["Value", config.value],
                    ["Description", config.description or "N/A"],
                    ["Created", self.adapter.format_datetime(config.created_at)],
                    ["Updated", self.adapter.format_datetime(config.updated_at)],
                ],
            )
        except Exception as e:
            self.adapter.print_error(f"Failed to get configuration: {e}")
            raise

    async def list(self, limit: int = 100, offset: int = 0) -> None:
        """
        List all configurations.
        """
        try:
            filter_dto = ConfigurationFilterDTO(limit=limit, offset=offset)
            configs = await self.adapter.list_configurations.execute(filter_dto)

            if not configs:
                self.adapter.print_info("No configurations found")
                return

            rows = [
                [
                    c.key,
                    str(c.value)[:50] + ("..." if len(str(c.value)) > 50 else ""),
                    (c.description or "")[:30]
                    + ("..." if c.description and len(c.description) > 30 else ""),
                    self.adapter.format_datetime(c.updated_at),
                ]
                for c in configs
            ]

            self.adapter.print_table(
                title=f"Configurations ({len(configs)} total)",
                columns=["Key", "Value", "Description", "Updated"],
                rows=rows,
            )
        except Exception as e:
            self.adapter.print_error(f"Failed to list configurations: {e}")
            raise
