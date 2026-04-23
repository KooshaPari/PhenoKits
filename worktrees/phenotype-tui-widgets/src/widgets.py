"""phenotype-tui-widgets - TUI widget library."""
from __future__ import annotations

from dataclasses import dataclass
from typing import Callable

__version__ = "0.1.0"


@dataclass
class Style:
    """Text styling."""
    fg: str | None = None
    bg: str | None = None
    bold: bool = False
    italic: bool = False
    underline: bool = False


class Widget:
    """Base widget class."""
    
    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height
        self.children: list[Widget] = []
        self.style = Style()
        self.visible = True

    def render(self) -> str:
        """Render widget to string."""
        raise NotImplementedError

    def handle_input(self, key: str) -> bool:
        """Handle input. Returns True if handled."""
        return False


class Text(Widget):
    """Text display widget."""

    def __init__(self, text: str, style: Style | None = None):
        super().__init__(len(text), 1)
        self.text = text
        self.style = style or Style()

    def render(self) -> str:
        """Render text."""
        return self.text


class Box(Widget):
    """Bordered box widget."""

    def __init__(self, child: Widget, title: str | None = None):
        super().__init__(child.width + 2, child.height + 2)
        self.child = child
        self.title = title

    def render(self) -> str:
        """Render box with border."""
        lines = []
        
        # Top border
        top = "┌" + "─" * (self.width - 2) + "┐"
        if self.title:
            title_line = f"┌─{self.title}─" + "─" * max(0, self.width - len(self.title) - 4) + "┐"
            lines.append(title_line)
        else:
            lines.append(top)
        
        # Content
        for line in self.child.render().split("\n"):
            lines.append(f"│ {line:<{self.width - 2}} │")
        
        # Bottom border
        lines.append("└" + "─" * (self.width - 2) + "┘")
        
        return "\n".join(lines)


class VStack(Widget):
    """Vertical stack of widgets."""

    def __init__(self, spacing: int = 1):
        super().__init__(0, 0)
        self.spacing = spacing
        self.children = []

    def add(self, widget: Widget) -> None:
        """Add widget to stack."""
        self.children.append(widget)
        self.width = max(w.width for w in self.children)
        self.height = sum(w.height for w in self.children) + self.spacing * (len(self.children) - 1)

    def render(self) -> str:
        """Render stacked widgets."""
        lines = []
        for widget in self.children:
            lines.append(widget.render())
        return "\n".join(lines)


class Button(Widget):
    """Button widget."""

    def __init__(self, label: str, on_click: Callable | None = None):
        super().__init__(len(label) + 4, 1)
        self.label = label
        self.on_click = on_click
        self.focused = False

    def render(self) -> str:
        """Render button."""
        prefix = "[" if self.focused else " "
        suffix = "]" if self.focused else " "
        return f"{prefix}{self.label}{suffix}"

    def handle_input(self, key: str) -> bool:
        """Handle button input."""
        if key == "enter" and self.on_click:
            self.on_click()
            return True
        return False


class List(Widget):
    """Selectable list widget."""

    def __init__(self, items: list[str], on_select: Callable | None = None):
        super().__init__(max(len(i) for i in items) if items else 0, len(items))
        self.items = items
        self.selected = 0
        self.on_select = on_select
        self.on_change: Callable | None = None

    def render(self) -> str:
        """Render list."""
        lines = []
        for i, item in enumerate(self.items):
            prefix = "▶ " if i == self.selected else "  "
            suffix = " ◀" if i == self.selected else ""
            lines.append(f"{prefix}{item}{suffix}")
        return "\n".join(lines)

    def handle_input(self, key: str) -> bool:
        """Handle list navigation."""
        if key == "up" and self.selected > 0:
            self.selected -= 1
            if self.on_change:
                self.on_change(self.selected)
            return True
        elif key == "down" and self.selected < len(self.items) - 1:
            self.selected += 1
            if self.on_change:
                self.on_change(self.selected)
            return True
        elif key == "enter" and self.on_select:
            self.on_select(self.selected, self.items[self.selected])
            return True
        return False


class ProgressBar(Widget):
    """Progress bar widget."""

    def __init__(self, width: int = 40):
        super().__init__(width, 1)
        self.width = width
        self.value = 0.0
        self.max = 100.0

    def set_progress(self, value: float, max_val: float | None = None) -> None:
        """Set progress value."""
        self.value = value
        if max_val is not None:
            self.max = max_val

    def render(self) -> str:
        """Render progress bar."""
        percent = min(self.value / self.max, 1.0)
        filled = int(self.width * percent)
        empty = self.width - filled
        return "[" + "█" * filled + "░" * empty + "]"


__all__ = [
    "Style",
    "Widget",
    "Text",
    "Box",
    "VStack",
    "Button",
    "List",
    "ProgressBar",
]
