"""
User CLI commands.
"""

from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from pheno.adapters.cli.adapter import CLIAdapter

from pheno.application.dtos.user import (
    CreateUserDTO,
    UpdateUserDTO,
    UserFilterDTO,
)


class UserCommands:
    """
    User management commands for CLI.
    """

    def __init__(self, adapter: CLIAdapter):
        self.adapter = adapter

    async def create(self, email: str, name: str) -> None:
        """
        Create a new user.
        """
        try:
            dto = CreateUserDTO(email=email, name=name)
            user = await self.adapter.create_user.execute(dto)
            self.adapter.print_success(f"User created: {user.name} ({user.email})")
            self.adapter.print_info(f"User ID: {user.id}")
        except Exception as e:
            self.adapter.print_error(f"Failed to create user: {e}")
            raise

    async def update(self, user_id: str, name: str | None = None, email: str | None = None) -> None:
        """
        Update a user.
        """
        try:
            dto = UpdateUserDTO(user_id=user_id, name=name, email=email)
            user = await self.adapter.update_user.execute(dto)
            self.adapter.print_success(f"User updated: {user.name} ({user.email})")
        except Exception as e:
            self.adapter.print_error(f"Failed to update user: {e}")
            raise

    async def get(self, user_id: str) -> None:
        """
        Get user details.
        """
        try:
            user = await self.adapter.get_user.execute(user_id)
            self.adapter.print_table(
                title=f"User: {user.name}",
                columns=["Field", "Value"],
                rows=[
                    ["ID", user.id],
                    ["Email", user.email],
                    ["Name", user.name],
                    ["Active", "Yes" if user.is_active else "No"],
                    ["Created", self.adapter.format_datetime(user.created_at)],
                    ["Updated", self.adapter.format_datetime(user.updated_at)],
                ],
            )
        except Exception as e:
            self.adapter.print_error(f"Failed to get user: {e}")
            raise

    async def list(self, limit: int = 100, offset: int = 0) -> None:
        """
        List all users.
        """
        try:
            filter_dto = UserFilterDTO(limit=limit, offset=offset)
            users = await self.adapter.list_users.execute(filter_dto)

            if not users:
                self.adapter.print_info("No users found")
                return

            rows = [
                [
                    user.id[:8] + "...",
                    user.email,
                    user.name,
                    "✓" if user.is_active else "✗",
                    self.adapter.format_datetime(user.created_at),
                ]
                for user in users
            ]

            self.adapter.print_table(
                title=f"Users ({len(users)} total)",
                columns=["ID", "Email", "Name", "Active", "Created"],
                rows=rows,
            )
        except Exception as e:
            self.adapter.print_error(f"Failed to list users: {e}")
            raise

    async def deactivate(self, user_id: str) -> None:
        """
        Deactivate a user.
        """
        try:
            user = await self.adapter.deactivate_user.execute(user_id)
            self.adapter.print_success(f"User deactivated: {user.name}")
        except Exception as e:
            self.adapter.print_error(f"Failed to deactivate user: {e}")
            raise
