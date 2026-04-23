"""
Service CLI commands.
"""

from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from pheno.adapters.cli.adapter import CLIAdapter

from pheno.application.dtos.service import (
    CreateServiceDTO,
    ServiceFilterDTO,
)


class ServiceCommands:
    """
    Service management commands for CLI.
    """

    def __init__(self, adapter: CLIAdapter):
        self.adapter = adapter

    async def create(self, name: str, port: int, protocol: str = "http") -> None:
        """
        Create a new service.
        """
        try:
            dto = CreateServiceDTO(name=name, port=port, protocol=protocol)
            service = await self.adapter.create_service.execute(dto)
            self.adapter.print_success(f"Service created: {service.name} (port: {service.port})")
            self.adapter.print_info(f"Service ID: {service.id}")
        except Exception as e:
            self.adapter.print_error(f"Failed to create service: {e}")
            raise

    async def start(self, service_id: str) -> None:
        """
        Start a service.
        """
        try:
            service = await self.adapter.start_service.execute(service_id)
            self.adapter.print_success(
                f"Service started: {service.name} (status: {service.status})",
            )
        except Exception as e:
            self.adapter.print_error(f"Failed to start service: {e}")
            raise

    async def stop(self, service_id: str) -> None:
        """
        Stop a service.
        """
        try:
            service = await self.adapter.stop_service.execute(service_id)
            self.adapter.print_success(f"Service stopped: {service.name}")
        except Exception as e:
            self.adapter.print_error(f"Failed to stop service: {e}")
            raise

    async def get(self, service_id: str) -> None:
        """
        Get service details.
        """
        try:
            service = await self.adapter.get_service.execute(service_id)
            self.adapter.print_table(
                title=f"Service: {service.name}",
                columns=["Field", "Value"],
                rows=[
                    ["ID", service.id],
                    ["Name", service.name],
                    ["Port", service.port],
                    ["Protocol", service.protocol],
                    ["Status", service.status],
                    ["Created", self.adapter.format_datetime(service.created_at)],
                    ["Updated", self.adapter.format_datetime(service.updated_at)],
                    ["Started", self.adapter.format_datetime(service.started_at)],
                    ["Stopped", self.adapter.format_datetime(service.stopped_at)],
                ],
            )
        except Exception as e:
            self.adapter.print_error(f"Failed to get service: {e}")
            raise

    async def list(self, limit: int = 100, offset: int = 0) -> None:
        """
        List all services.
        """
        try:
            filter_dto = ServiceFilterDTO(limit=limit, offset=offset)
            services = await self.adapter.list_services.execute(filter_dto)

            if not services:
                self.adapter.print_info("No services found")
                return

            rows = [
                [
                    s.id[:8] + "...",
                    s.name,
                    s.port,
                    s.protocol,
                    s.status,
                    self.adapter.format_datetime(s.created_at),
                ]
                for s in services
            ]

            self.adapter.print_table(
                title=f"Services ({len(services)} total)",
                columns=["ID", "Name", "Port", "Protocol", "Status", "Created"],
                rows=rows,
            )
        except Exception as e:
            self.adapter.print_error(f"Failed to list services: {e}")
            raise

    async def health(self) -> None:
        """
        Get service health status.
        """
        try:
            health = await self.adapter.get_service_health.execute()
            self.adapter.print_table(
                title="Service Health Status",
                columns=["Metric", "Value"],
                rows=[
                    ["Total Services", health.total_services],
                    ["Running", health.running],
                    ["Stopped", health.stopped],
                    ["Failed", health.failed],
                    ["Health %", f"{health.healthy_percentage:.1f}%"],
                ],
            )

            # Print health indicator
            if health.healthy_percentage >= 90:
                self.adapter.print_success("System health: EXCELLENT")
            elif health.healthy_percentage >= 70:
                self.adapter.print_info("System health: GOOD")
            elif health.healthy_percentage >= 50:
                self.adapter.print_warning("System health: DEGRADED")
            else:
                self.adapter.print_error("System health: CRITICAL")
        except Exception as e:
            self.adapter.print_error(f"Failed to get service health: {e}")
            raise
