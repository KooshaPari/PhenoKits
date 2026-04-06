"""
Deployment CLI commands.
"""

from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from pheno.adapters.cli.adapter import CLIAdapter

from pheno.application.dtos.deployment import (
    CreateDeploymentDTO,
    DeploymentFilterDTO,
)


class DeploymentCommands:
    """
    Deployment management commands for CLI.
    """

    def __init__(self, adapter: CLIAdapter):
        self.adapter = adapter

    async def create(self, environment: str, strategy: str) -> None:
        """
        Create a new deployment.
        """
        try:
            dto = CreateDeploymentDTO(environment=environment, strategy=strategy)
            deployment = await self.adapter.create_deployment.execute(dto)
            self.adapter.print_success(
                f"Deployment created: {deployment.environment} ({deployment.strategy})",
            )
            self.adapter.print_info(f"Deployment ID: {deployment.id}")
        except Exception as e:
            self.adapter.print_error(f"Failed to create deployment: {e}")
            raise

    async def start(self, deployment_id: str) -> None:
        """
        Start a deployment.
        """
        try:
            deployment = await self.adapter.start_deployment.execute(deployment_id)
            self.adapter.print_success(
                f"Deployment started: {deployment.id} (status: {deployment.status})",
            )
        except Exception as e:
            self.adapter.print_error(f"Failed to start deployment: {e}")
            raise

    async def complete(self, deployment_id: str) -> None:
        """
        Complete a deployment.
        """
        try:
            deployment = await self.adapter.complete_deployment.execute(deployment_id)
            self.adapter.print_success(f"Deployment completed: {deployment.id}")
        except Exception as e:
            self.adapter.print_error(f"Failed to complete deployment: {e}")
            raise

    async def fail(self, deployment_id: str, reason: str) -> None:
        """
        Fail a deployment.
        """
        try:
            deployment = await self.adapter.fail_deployment.execute(deployment_id, reason)
            self.adapter.print_warning(f"Deployment failed: {deployment.id}")
            self.adapter.print_info(f"Reason: {reason}")
        except Exception as e:
            self.adapter.print_error(f"Failed to mark deployment as failed: {e}")
            raise

    async def rollback(self, deployment_id: str, reason: str) -> None:
        """
        Rollback a deployment.
        """
        try:
            deployment = await self.adapter.rollback_deployment.execute(deployment_id, reason)
            self.adapter.print_warning(f"Deployment rolled back: {deployment.id}")
            self.adapter.print_info(f"Reason: {reason}")
        except Exception as e:
            self.adapter.print_error(f"Failed to rollback deployment: {e}")
            raise

    async def get(self, deployment_id: str) -> None:
        """
        Get deployment details.
        """
        try:
            deployment = await self.adapter.get_deployment.execute(deployment_id)
            self.adapter.print_table(
                title=f"Deployment: {deployment.id[:8]}...",
                columns=["Field", "Value"],
                rows=[
                    ["ID", deployment.id],
                    ["Environment", deployment.environment],
                    ["Strategy", deployment.strategy],
                    ["Status", deployment.status],
                    ["Created", self.adapter.format_datetime(deployment.created_at)],
                    ["Updated", self.adapter.format_datetime(deployment.updated_at)],
                    ["Started", self.adapter.format_datetime(deployment.started_at)],
                    [
                        "Completed",
                        self.adapter.format_datetime(deployment.completed_at),
                    ],
                ],
            )
        except Exception as e:
            self.adapter.print_error(f"Failed to get deployment: {e}")
            raise

    async def list(self, limit: int = 100, offset: int = 0) -> None:
        """
        List all deployments.
        """
        try:
            filter_dto = DeploymentFilterDTO(limit=limit, offset=offset)
            deployments = await self.adapter.list_deployments.execute(filter_dto)

            if not deployments:
                self.adapter.print_info("No deployments found")
                return

            rows = [
                [
                    d.id[:8] + "...",
                    d.environment,
                    d.strategy,
                    d.status,
                    self.adapter.format_datetime(d.created_at),
                ]
                for d in deployments
            ]

            self.adapter.print_table(
                title=f"Deployments ({len(deployments)} total)",
                columns=["ID", "Environment", "Strategy", "Status", "Created"],
                rows=rows,
            )
        except Exception as e:
            self.adapter.print_error(f"Failed to list deployments: {e}")
            raise

    async def statistics(self, environment: str | None = None) -> None:
        """
        Get deployment statistics.
        """
        try:
            stats = await self.adapter.get_deployment_statistics.execute(environment)
            self.adapter.print_table(
                title=f"Deployment Statistics{f' ({environment})' if environment else ''}",
                columns=["Metric", "Count"],
                rows=[
                    ["Total", stats.total],
                    ["Pending", stats.pending],
                    ["In Progress", stats.in_progress],
                    ["Completed", stats.completed],
                    ["Failed", stats.failed],
                    ["Rolled Back", stats.rolled_back],
                ],
            )
        except Exception as e:
            self.adapter.print_error(f"Failed to get deployment statistics: {e}")
            raise
