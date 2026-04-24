# ADR-004: Immediate-Mode TUI with Textual

## Status
**ACCEPTED**

## Context

PhenoLang's pheno-ui and pheno-tui modules provide developer interfaces for infrastructure management. Current terminal interfaces:

- **Static Output**: Printf-style logging, hard to scan
- **No Interactivity**: Long-running operations provide no feedback
- **Poor UX**: Error conditions buried in log output
- **State Visibility**: Resource status changes not visible in real-time

With 33 modules and complex orchestration workflows, developers need rich, responsive interfaces that provide:
- Real-time deployment status
- Interactive resource exploration
- Command palette for quick actions
- Split-pane log viewing
- Keyboard-driven workflows

## Decision

We will adopt **Textual** for all new terminal user interfaces, implementing an **immediate-mode reactive architecture**.

### Decision Details

#### Textual Component Model

Textual implements a React-like reactive component system for terminals:

```python
from textual.app import App, ComposeResult
from textual.containers import Container, Grid, Horizontal, Vertical
from textual.widgets import (
    Header, Footer, Static, Button, Input, DataTable, 
    Tree, Log, ProgressBar, Label
)
from textual.reactive import reactive
from textual.binding import Binding
from textual.screen import ModalScreen

class DeploymentDashboard(App):
    """Main dashboard for deployment orchestration."""
    
    # Reactive state - automatic UI updates on change
    resources: reactive[list] = reactive([])
    selected_resource: reactive[Optional[Resource]] = reactive(None)
    deployment_status: reactive[str] = reactive("idle")
    logs: reactive[list] = reactive([])
    
    CSS = """
    Screen { align: center middle; }
    
    #sidebar {
        width: 25%;
        height: 100%;
        border: solid $primary;
    }
    
    #main-content {
        width: 75%;
        height: 100%;
    }
    
    .status-badge {
        padding: 0 1;
    }
    
    .status-running { background: $success; color: $text; }
    .status-pending { background: $warning; color: $text; }
    .status-failed { background: $error; color: $text-dark; }
    .status-stopped { background: $surface; color: $text-muted; }
    
    ResourceCard {
        height: auto;
        padding: 1;
        border: solid $surface-lighten-1;
        margin: 0 0 1 0;
    }
    
    ResourceCard:hover {
        border: solid $primary;
    }
    
    #log-panel {
        height: 30%;
        border-top: solid $primary;
    }
    
    DataTable {
        height: 100%;
    }
    """
    
    BINDINGS = [
        Binding("q", "quit", "Quit", priority=True),
        Binding("r", "refresh", "Refresh", show=True),
        Binding("d", "deploy", "Deploy", show=True),
        Binding("s", "stop", "Stop", show=True),
        Binding("l", "logs", "View Logs", show=True),
        Binding("?", "help", "Help", show=True),
        Binding("/", "search", "Search", show=True),
        Binding("1", "focus_resources", "Resources", show=False),
        Binding("2", "focus_logs", "Logs", show=False),
    ]
    
    def compose(self) -> ComposeResult:
        yield Header(show_clock=True)
        
        with Horizontal():
            # Left sidebar: Resource tree
            with Vertical(id="sidebar"):
                yield Label("Resources", classes="sidebar-header")
                yield Tree("Deployments", id="resource-tree")
                yield Label("Filter", classes="sidebar-header")
                yield Input(placeholder="Type to filter...", id="filter-input")
            
            # Main content area
            with Vertical(id="main-content"):
                # Top: Status bar
                with Horizontal(classes="status-bar"):
                    yield Label(id="status-label")
                    yield ProgressBar(total=100, id="progress-bar")
                
                # Middle: Resource grid
                with Grid(id="resource-grid"):
                    for resource in self.resources:
                        yield ResourceCard(resource)
                
                # Bottom: Log panel
                with Container(id="log-panel"):
                    yield Log(id="deployment-logs", max_lines=1000)
        
        yield Footer()
    
    # Reactive watchers - UI updates automatically
    def watch_resources(self, resources: list) -> None:
        """Update resource grid when resources change."""
        grid = self.query_one("#resource-grid", Grid)
        grid.remove_children()
        for resource in resources:
            grid.mount(ResourceCard(resource))
    
    def watch_deployment_status(self, status: str) -> None:
        """Update status display."""
        label = self.query_one("#status-label", Label)
        label.update(f"Status: {status.upper()}")
        
        # Update progress bar based on status
        progress = self.query_one("#progress-bar", ProgressBar)
        if status == "deploying":
            progress.update(progress=50)
        elif status == "running":
            progress.update(progress=100)
        elif status == "failed":
            progress.update(progress=0)
    
    def watch_logs(self, logs: list) -> None:
        """Append new logs."""
        log_widget = self.query_one("#deployment-logs", Log)
        for log in logs[-10:]:  # Show last 10
            log_widget.write_line(f"[{log.timestamp}] {log.level}: {log.message}")
    
    # Actions
    async def action_deploy(self) -> None:
        """Start deployment."""
        self.deployment_status = "deploying"
        
        # Show confirmation modal
        confirmed = await self.push_screen_wait(DeployConfirmationScreen())
        if not confirmed:
            self.deployment_status = "idle"
            return
        
        # Execute deployment
        try:
            await self._execute_deployment()
            self.deployment_status = "running"
            self.notify("Deployment successful!", severity="information")
        except Exception as e:
            self.deployment_status = "failed"
            self.notify(f"Deployment failed: {e}", severity="error")
    
    async def action_search(self) -> None:
        """Show search modal."""
        result = await self.push_screen_wait(SearchScreen(self.resources))
        if result:
            self.selected_resource = result
            self._focus_resource(result)
    
    def action_focus_resources(self) -> None:
        """Focus resource tree."""
        self.query_one("#resource-tree", Tree).focus()
    
    def action_focus_logs(self) -> None:
        """Focus log panel."""
        self.query_one("#deployment-logs", Log).focus()

class ResourceCard(Static):
    """Card widget displaying resource status."""
    
    resource: reactive[Resource] = reactive(None)
    
    def __init__(self, resource: Resource, **kwargs):
        super().__init__(**kwargs)
        self.resource = resource
    
    def compose(self) -> ComposeResult:
        with Container():
            yield Label(self.resource.name, classes="resource-name")
            yield Label(self.resource.type, classes="resource-type")
            with Container(classes=f"status-badge status-{self.resource.status}"):
                yield Label(self.resource.status.upper())
    
    def watch_resource(self, resource: Resource) -> None:
        """Update card when resource changes."""
        self.update_styles()
        self.refresh()
    
    def on_click(self) -> None:
        """Handle selection."""
        self.app.selected_resource = self.resource
        self.app.push_screen(ResourceDetailScreen(self.resource))

class ResourceDetailScreen(ModalScreen):
    """Modal screen showing resource details."""
    
    def __init__(self, resource: Resource, **kwargs):
        super().__init__(**kwargs)
        self.resource = resource
    
    def compose(self) -> ComposeResult:
        with Container(id="detail-container"):
            yield Label(f"Resource: {self.resource.name}", classes="title")
            yield Label(f"Type: {self.resource.type}")
            yield Label(f"Status: {self.resource.status}")
            yield Label(f"Region: {self.resource.region}")
            yield Label(f"Created: {self.resource.created_at}")
            
            with Horizontal(classes="actions"):
                yield Button("Start", id="btn-start", variant="primary")
                yield Button("Stop", id="btn-stop", variant="warning")
                yield Button("Delete", id="btn-delete", variant="error")
                yield Button("Close", id="btn-close", variant="default")
    
    def on_button_pressed(self, event: Button.Pressed) -> None:
        """Handle action buttons."""
        if event.button.id == "btn-close":
            self.dismiss()
        elif event.button.id == "btn-start":
            self._start_resource()
        elif event.button.id == "btn-stop":
            self._stop_resource()
        elif event.button.id == "btn-delete":
            self._confirm_delete()

class DeployConfirmationScreen(ModalScreen[bool]):
    """Confirmation dialog for deployment."""
    
    CSS = """
    #dialog {
        grid-size: 2;
        grid-gutter: 1 2;
        padding: 0 1;
        width: 60;
        height: 11;
        border: thick $background 80%;
        background: $surface;
    }
    
    #question {
        column-span: 2;
        height: 1fr;
        width: 1fr;
        content-align: center middle;
    }
    """
    
    def compose(self) -> ComposeResult:
        with Container(id="dialog"):
            yield Label("Start deployment? This will provision resources.", id="question")
            yield Button("Deploy", variant="primary", id="deploy")
            yield Button("Cancel", id="cancel")
    
    def on_button_pressed(self, event: Button.Pressed) -> None:
        if event.button.id == "deploy":
            self.dismiss(True)
        else:
            self.dismiss(False)

class SearchScreen(ModalScreen[Optional[Resource]]):
    """Fuzzy search for resources."""
    
    def __init__(self, resources: list, **kwargs):
        super().__init__(**kwargs)
        self.all_resources = resources
        self.filtered = resources[:]
    
    def compose(self) -> ComposeResult:
        with Container():
            yield Input(placeholder="Search resources...", id="search-input")
            yield ListView(id="search-results")
    
    def on_input_changed(self, event: Input.Changed) -> None:
        """Filter resources as user types."""
        query = event.value.lower()
        self.filtered = [
            r for r in self.all_resources
            if query in r.name.lower() or query in r.type.lower()
        ]
        
        list_view = self.query_one("#search-results", ListView)
        list_view.clear()
        for resource in self.filtered[:20]:  # Limit results
            list_view.append(ListItem(Label(f"{r.name} ({r.type})", data=resource)))
    
    def on_list_view_selected(self, event: ListView.Selected) -> None:
        """Return selected resource."""
        resource = event.item.children[0].data
        self.dismiss(resource)
```

### Custom Widget Development

```python
from textual.widgets import Widget
from textual.message import Message
from textual.reactive import reactive

class ResourceTimeline(Widget):
    """Timeline widget showing resource events."""
    
    events: reactive[list] = reactive([])
    
    CSS = """
    ResourceTimeline {
        height: auto;
        padding: 1;
    }
    
    .timeline-item {
        height: 3;
        layout: horizontal;
    }
    
    .timeline-marker {
        width: 3;
        content-align: center middle;
    }
    
    .timeline-content {
        width: 1fr;
    }
    """
    
    def render(self) -> str:
        """Render timeline as Rich renderable."""
        from rich.table import Table
        from rich.text import Text
        
        table = Table(show_header=False, box=None, padding=0)
        table.add_column("marker", width=3)
        table.add_column("content")
        
        for i, event in enumerate(self.events):
            marker = "●" if i == len(self.events) - 1 else "○"
            color = self._event_color(event)
            
            content = Text()
            content.append(f"{event.timestamp:%H:%M:%S} ", style="dim")
            content.append(event.type, style="bold")
            content.append(f" - {event.description}")
            
            table.add_row(
                Text(marker, style=color),
                content
            )
        
        return table
    
    def _event_color(self, event: TimelineEvent) -> str:
        colors = {
            "created": "green",
            "provisioned": "blue",
            "started": "green",
            "failed": "red",
            "stopped": "yellow",
        }
        return colors.get(event.type, "white")

class MetricsSparkline(Widget):
    """Sparkline widget for metrics visualization."""
    
    data: reactive[list[float]] = reactive([])
    max_points: int = 100
    
    CSS = """
    MetricsSparkline {
        height: 3;
        border: solid $primary;
        padding: 0 1;
    }
    """
    
    def watch_data(self, data: list) -> None:
        """Trim to max points."""
        if len(data) > self.max_points:
            self.data = data[-self.max_points:]
    
    def render(self) -> str:
        """Render sparkline."""
        if not self.data:
            return "No data"
        
        from rich.sparkline import Sparkline
        return Sparkline(self.data, width=self.size.width - 2)
    
    def add_point(self, value: float) -> None:
        """Add new data point."""
        self.data = [*self.data, value]
```

### DataTable for Structured Data

```python
class ResourceTable(DataTable):
    """Enhanced DataTable with sorting and filtering."""
    
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self._all_rows: list = []
        self._sort_column: Optional[str] = None
        self._sort_reverse: bool = False
    
    def on_mount(self) -> None:
        """Initialize columns."""
        self.add_columns(
            "Name",
            "Type",
            "Status",
            "Region",
            "Health",
            "Created",
        )
        self.cursor_type = "row"
        self.zebra_stripes = True
        self.fixed_columns = 1  # Keep name column visible
    
    def add_resource(self, resource: Resource) -> None:
        """Add resource row with metadata."""
        row_key = self.add_row(
            resource.name,
            resource.type,
            resource.status,
            resource.region,
            resource.health_indicator,
            resource.created_at.strftime("%Y-%m-%d %H:%M"),
        )
        self._all_rows.append((row_key, resource))
    
    def filter(self, query: str) -> None:
        """Filter rows by query."""
        query = query.lower()
        
        # Clear and re-add matching rows
        self.clear()
        for row_key, resource in self._all_rows:
            if query in resource.name.lower() or query in resource.type.lower():
                self.add_resource(resource)
    
    def sort_by(self, column: str) -> None:
        """Sort by column."""
        if self._sort_column == column:
            self._sort_reverse = not self._sort_reverse
        else:
            self._sort_column = column
            self._sort_reverse = False
        
        # Sort and refresh
        sort_key = lambda r: getattr(r[1], column, "")
        self._all_rows.sort(key=sort_key, reverse=self._sort_reverse)
        
        self.clear()
        for _, resource in self._all_rows:
            self.add_resource(resource)
    
    def on_data_table_header_selected(self, event: DataTable.HeaderSelected) -> None:
        """Sort on header click."""
        column = self.columns[event.column_index].label
        self.sort_by(column)
    
    def on_data_table_row_selected(self, event: DataTable.RowSelected) -> None:
        """Emit selection event."""
        # Find resource for selected row
        for row_key, resource in self._all_rows:
            if row_key == event.row_key:
                self.post_message(self.Selected(resource))
                break
    
    class Selected(Message):
        """Resource selection message."""
        
        def __init__(self, resource: Resource) -> None:
            self.resource = resource
            super().__init__()
```

## Consequences

### Positive
- **Rich UX**: Modern terminal interfaces with layouts, colors, animations
- **Reactivity**: State changes automatically update UI
- **Keyboard-Driven**: Efficient workflows for power users
- **Responsive**: Immediate feedback on user actions
- **Testable**: Component-based architecture enables unit testing
- **Maintainable**: CSS-like styling, declarative composition

### Negative
- **Python 3.8+ Required**: Textual requires modern Python
- **Learning Curve**: Team must learn reactive patterns
- **Terminal Requirements**: Rich output needs modern terminal emulators
- **Bundle Size**: Additional dependency for CLI tools
- **Performance**: Large data sets may need virtualization

### Mitigations

1. **Fallback**: Provide `--plain` flag for basic terminal support
2. **Documentation**: Comprehensive component guide in `docs/ui/`
3. **Testing**: Use `textual` test framework for automated UI testing
4. **Performance**: Implement virtual scrolling for large tables

## Implementation Plan

### Phase 1: Foundation (Week 1)
- [ ] Create base widget library in pheno-ui
- [ ] Implement common components (ResourceCard, StatusBadge, etc.)
- [ ] Define CSS design system
- [ ] Create dashboard layout template

### Phase 2: Migration (Weeks 2-3)
- [ ] Convert pheno-tui main interface
- [ ] Implement deployment dashboard
- [ ] Add resource explorer
- [ ] Create log viewer

### Phase 3: Enhancement (Week 4)
- [ ] Add search functionality
- [ ] Implement command palette
- [ ] Add keyboard shortcuts documentation
- [ ] Performance optimization

## Alternatives Considered

### Alternative 1: Rich (non-interactive)
- **Pros**: Excellent for output formatting, widely used
- **Cons**: No interactivity, static output only
- **Verdict**: Rejected as primary; use for non-interactive output

### Alternative 2: prompt-toolkit
- **Pros**: Mature, extensive widget library
- **Cons**: Event-driven (not reactive), steeper learning curve
- **Verdict**: Rejected; prefer reactive model

### Alternative 3: blessed
- **Pros**: Low-level control, minimal dependencies
- **Cons**: Verbose, manual layout management
- **Verdict**: Rejected; too low-level for productivity

## Related Decisions
- ADR-002: Domain-Driven Design (state management)
- ADR-003: Structured Concurrency (async operations)
- ADR-005: Code Generation (UI scaffolding)

## References
- Textual Documentation: https://textual.textualize.io/
- Rich Documentation: https://rich.readthedocs.io/
- prompt-toolkit Documentation: https://python-prompt-toolkit.readthedocs.io/
