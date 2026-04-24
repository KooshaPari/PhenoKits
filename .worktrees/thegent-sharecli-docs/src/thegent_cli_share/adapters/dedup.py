"""In-memory lock adapter for command deduplication."""

from typing import Optional
from ..domain.entities import CommandLock, LockStatus
from ..domain.value_objects import CommandHash


class InMemoryLockAdapter:
    """In-memory implementation of LockPort for testing and local use."""

    def __init__(self) -> None:
        self._locks: dict[str, CommandLock] = {}

    def acquire(
        self, cmd_hash: str, pid: int, output_path: Optional[str] = None
    ) -> CommandLock:
        """Acquire a command lock."""
        if cmd_hash in self._locks:
            lock = self._locks[cmd_hash]
            if lock.is_locked() and lock.pid != pid:
                raise ValueError("already locked")
            lock.acquire(pid, output_path)
            return lock
        else:
            lock = CommandLock(cmd_hash=cmd_hash, pid=pid, output_path=output_path)
            lock.status = LockStatus.LOCKED
            self._locks[cmd_hash] = lock
            return lock

    def release(self, cmd_hash: str, pid: int) -> None:
        """Release a command lock."""
        if cmd_hash not in self._locks:
            raise ValueError(f"No lock found for {cmd_hash}")
        lock = self._locks[cmd_hash]
        lock.release(pid)

    def get(self, cmd_hash: str) -> Optional[CommandLock]:
        """Get lock status."""
        return self._locks.get(cmd_hash)

    def list_all(self) -> list[CommandLock]:
        """List all locks."""
        return list(self._locks.values())


# Re-export CommandLock for convenience
from ..domain.entities import CommandLock
__all__ = ["InMemoryLockAdapter", "CommandLock"]
