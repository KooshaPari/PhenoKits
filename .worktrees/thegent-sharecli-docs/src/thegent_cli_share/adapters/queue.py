"""In-memory queue adapter for task queue operations."""

from enum import Enum
from typing import Optional, NamedTuple
from uuid import uuid4
from ..domain.entities import TaskQueueItem, QueuePriority


class Priority(str, Enum):
    """Task priority levels."""
    LOW = "low"
    NORMAL = "normal"
    HIGH = "high"
    CRITICAL = "critical"


class QueueItem(NamedTuple):
    """Simple queue item for testing."""
    id: str
    command: str
    status: str
    priority: Priority


class InMemoryQueueAdapter:
    """In-memory implementation of QueuePort for testing and local use."""

    def __init__(self) -> None:
        self._queue: list[QueueItem] = []
        self._items: dict[str, TaskQueueItem] = {}

    def enqueue(
        self,
        command: str,
        priority: Priority = Priority.NORMAL,
        **kwargs
    ) -> QueueItem:
        """Add item to queue."""
        item_id = str(uuid4())
        item = QueueItem(
            id=item_id,
            command=command,
            status="queued",
            priority=priority,
        )
        self._queue.append(item)
        self._items[item_id] = TaskQueueItem(
            id=uuid4(),
            command=command,
            priority=QueuePriority(priority.value),
            status="queued",
            **kwargs
        )
        # Sort by priority
        self._queue.sort(key=lambda x: (
            0 if x.priority == Priority.CRITICAL else
            1 if x.priority == Priority.HIGH else
            2 if x.priority == Priority.NORMAL else 3
        ))
        return item

    def dequeue(self) -> Optional[QueueItem]:
        """Remove and return next item."""
        if not self._queue:
            return None
        item = self._queue.pop(0)
        # Update status in items dict
        task_item = self._items.get(item.id)
        if task_item:
            task_item.status = "dequeued"
        return QueueItem(item.id, item.command, "dequeued", item.priority)

    def peek(self) -> Optional[QueueItem]:
        """View next item without removing."""
        if not self._queue:
            return None
        return self._queue[0]

    def length(self) -> int:
        """Get queue length."""
        return len(self._queue)

    def clear(self) -> None:
        """Clear the queue."""
        self._queue.clear()
        self._items.clear()


__all__ = ["InMemoryQueueAdapter", "QueueItem", "Priority"]
