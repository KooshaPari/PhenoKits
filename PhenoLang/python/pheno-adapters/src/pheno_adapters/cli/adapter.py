"""CLI Adapter implementation.

This adapter provides a bridge between CLI commands and application use cases. It
follows the hexagonal architecture pattern by depending on ports (interfaces) rather
than concrete implementations.
"""

from __future__ import annotations

from typing import TYPE_CHECKING, Any

from rich.console import Console
from rich.table import Table

from pheno.application.use_cases import (
    CompleteDeploymentUseCase,
    CreateConfigurationUseCase,
    CreateDeploymentUseCase,
    CreateServiceUseCase,
    CreateUserUseCase,
    DeactivateUserUseCase,
    FailDeploymentUseCase,
    GetConfigurationUseCase,
    GetDeploymentStatisticsUseCase,
    GetDeploymentUseCase,
    GetServiceHealthUseCase,
    GetServiceUseCase,
    GetUserUseCase,
    ListConfigurationsUseCase,
    ListDeploymentsUseCase,
    ListServicesUseCase,
    ListUsersUseCase,
    RollbackDeploymentUseCase,
    StartDeploymentUseCase,
    StartServiceUseCase,
    StopServiceUseCase,
    UpdateConfigurationUseCase,
    UpdateUserUseCase,
)

if TYPE_CHECKING:
    from pheno.application.ports.events import EventPublisher
    from pheno.application.ports.repositories import (
        ConfigurationRepository,
        DeploymentRepository,
        ServiceRepository,
        UserRepository,
    )


class CLIAdapter:
    """CLI Adapter for translating CLI commands to use cases.

    This adapter is responsible for:
    - Input validation and transformation
    - Calling appropriate use cases
    - Formatting output for CLI display
    - Error handling and user feedback
    """

    def __init__(
        self,
        user_repository: UserRepository,
        deployment_repository: DeploymentRepository,
        service_repository: ServiceRepository,
        configuration_repository: ConfigurationRepository,
        event_publisher: EventPublisher,
    ):
        """
        Initialize CLI adapter with required dependencies.
        """
        self.console = Console()

        # User use cases
        self.create_user = CreateUserUseCase(user_repository, event_publisher)
        self.update_user = UpdateUserUseCase(user_repository, event_publisher)
        self.get_user = GetUserUseCase(user_repository)
        self.list_users = ListUsersUseCase(user_repository)
        self.deactivate_user = DeactivateUserUseCase(user_repository, event_publisher)

        # Deployment use cases
        self.create_deployment = CreateDeploymentUseCase(deployment_repository, event_publisher)
        self.start_deployment = StartDeploymentUseCase(deployment_repository, event_publisher)
        self.complete_deployment = CompleteDeploymentUseCase(deployment_repository, event_publisher)
        self.fail_deployment = FailDeploymentUseCase(deployment_repository, event_publisher)
        self.rollback_deployment = RollbackDeploymentUseCase(deployment_repository, event_publisher)
        self.get_deployment = GetDeploymentUseCase(deployment_repository)
        self.list_deployments = ListDeploymentsUseCase(deployment_repository)
        self.get_deployment_statistics = GetDeploymentStatisticsUseCase(deployment_repository)

        # Service use cases
        self.create_service = CreateServiceUseCase(service_repository, event_publisher)
        self.start_service = StartServiceUseCase(service_repository, event_publisher)
        self.stop_service = StopServiceUseCase(service_repository, event_publisher)
        self.get_service = GetServiceUseCase(service_repository)
        self.list_services = ListServicesUseCase(service_repository)
        self.get_service_health = GetServiceHealthUseCase(service_repository)

        # Configuration use cases
        self.create_configuration = CreateConfigurationUseCase(configuration_repository)
        self.update_configuration = UpdateConfigurationUseCase(configuration_repository)
        self.get_configuration = GetConfigurationUseCase(configuration_repository)
        self.list_configurations = ListConfigurationsUseCase(configuration_repository)

    def print_success(self, message: str) -> None:
        """
        Print success message.
        """
        self.console.print(f"[green]✓[/green] {message}")

    def print_error(self, message: str) -> None:
        """
        Print error message.
        """
        self.console.print(f"[red]✗[/red] {message}")

    def print_info(self, message: str) -> None:
        """
        Print info message.
        """
        self.console.print(f"[blue]ℹ[/blue] {message}")

    def print_warning(self, message: str) -> None:
        """
        Print warning message.
        """
        self.console.print(f"[yellow]⚠[/yellow] {message}")

    def print_table(self, title: str, columns: list[str], rows: list[list[Any]]) -> None:
        """
        Print a formatted table.
        """
        table = Table(title=title)
        for column in columns:
            table.add_column(column)
        for row in rows:
            table.add_row(*[str(cell) for cell in row])
        self.console.print(table)

    def format_datetime(self, dt: Any) -> str:
        """
        Format datetime for display.
        """
        if dt is None:
            return "N/A"
        return dt.strftime("%Y-%m-%d %H:%M:%S")
