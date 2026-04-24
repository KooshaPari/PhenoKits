import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";

import { saveSettings } from "../../api/settings";
import { useProjects } from "../../hooks/useProjects";
import { SettingsView } from "../../views/SettingsView";

// Traces to: FR-TRACERA-VIEWS-007
vi.mock("../../api/settings", () => ({
  saveSettings: vi.fn().mockResolvedValue({}),
}));

vi.mock("../../hooks/useProjects", () => ({
  useProjects: vi.fn(),
}));

describe(SettingsView, () => {
  let queryClient: QueryClient;

  beforeEach(() => {
    queryClient = new QueryClient({
      defaultOptions: {
        mutations: { retry: false },
        queries: { retry: false },
      },
    });
    vi.clearAllMocks();
    vi.mocked(useProjects).mockReturnValue({
      data: [],
      error: null,
      isError: false,
      isLoading: false,
    } as any);
  });

  it("renders settings interface", () => {
    render(
      <QueryClientProvider client={queryClient}>
        <SettingsView />
      </QueryClientProvider>,
    );

    expect(screen.getByText("System Preferences")).toBeInTheDocument();
    expect(screen.getByRole("tab", { name: /Identity/i })).toBeInTheDocument();
    expect(screen.getByRole("tab", { name: /Visuals/i })).toBeInTheDocument();
    expect(screen.getByRole("tab", { name: /Engine Access/i })).toBeInTheDocument();
    expect(screen.getByRole("tab", { name: /Comms/i })).toBeInTheDocument();
  });

  it("displays general settings", () => {
    render(
      <QueryClientProvider client={queryClient}>
        <SettingsView />
      </QueryClientProvider>,
    );

    expect(screen.getByText("Public Identity")).toBeInTheDocument();
    expect(screen.getByText("Registry Name")).toBeInTheDocument();
    expect(screen.getByText("Direct Comms (Email)")).toBeInTheDocument();
    expect(screen.getByText("Timezone / Location Context")).toBeInTheDocument();
  });

  it("displays appearance settings", async () => {
    const user = userEvent.setup();
    render(
      <QueryClientProvider client={queryClient}>
        <SettingsView />
      </QueryClientProvider>,
    );

    // Click on Appearance tab
    await user.click(screen.getByRole("tab", { name: /Visuals/i }));

    expect(screen.getByText("Interface Directives")).toBeInTheDocument();
    expect(screen.getByText("Color Schema")).toBeInTheDocument();
    expect(screen.getByText("Information Density")).toBeInTheDocument();
  });

  it("displays API keys settings", async () => {
    const user = userEvent.setup();
    render(
      <QueryClientProvider client={queryClient}>
        <SettingsView />
      </QueryClientProvider>,
    );

    // Click on API Keys tab
    await user.click(screen.getByRole("tab", { name: /Engine Access/i }));

    expect(screen.getByText("Engine Interface Access")).toBeInTheDocument();
    expect(screen.getByText("Master API Link")).toBeInTheDocument();
    expect(screen.getByText("REGEN")).toBeInTheDocument();
  });

  it("displays notification settings", async () => {
    const user = userEvent.setup();
    render(
      <QueryClientProvider client={queryClient}>
        <SettingsView />
      </QueryClientProvider>,
    );

    // Click on Notifications tab
    await user.click(screen.getByRole("tab", { name: /Comms/i }));

    expect(screen.getByText("Telemetry & Comms")).toBeInTheDocument();
    expect(screen.getByText("Email Dispatches")).toBeInTheDocument();
    expect(screen.getByText("Desktop Stream")).toBeInTheDocument();
    expect(screen.getByText("Executive Weekly")).toBeInTheDocument();
  });

  it("handles theme selection", async () => {
    const user = userEvent.setup();
    render(
      <QueryClientProvider client={queryClient}>
        <SettingsView />
      </QueryClientProvider>,
    );

    // Click on Appearance tab
    await user.click(screen.getByRole("tab", { name: /Visuals/i }));

    const darkThemeButton = screen.getByRole("button", { name: "dark" });
    await user.click(darkThemeButton);

    await waitFor(() => {
      expect(darkThemeButton).toHaveClass("bg-primary");
    });
  });

  it("handles notification toggles", async () => {
    const user = userEvent.setup();
    render(
      <QueryClientProvider client={queryClient}>
        <SettingsView />
      </QueryClientProvider>,
    );

    // Click on Notifications tab
    await user.click(screen.getByRole("tab", { name: /Comms/i }));

    const emailNotifications = screen.getAllByRole("checkbox")[0];
    expect(emailNotifications).toBeChecked();

    await user.click(emailNotifications);
    await waitFor(() => {
      expect(emailNotifications).not.toBeChecked();
    });
  });

  it("saves settings when save button is clicked", async () => {
    const user = userEvent.setup();
    render(
      <QueryClientProvider client={queryClient}>
        <SettingsView />
      </QueryClientProvider>,
    );

    const displayNameInput = screen.getByDisplayValue("System Administrator");
    await user.clear(displayNameInput);
    await user.type(displayNameInput, "Test User");

    const saveButton = screen.getByRole("button", { name: /Synchronize Parameters/i });
    expect(saveButton).toBeInTheDocument();

    await user.click(saveButton);

    await waitFor(() => {
      expect(saveSettings).toHaveBeenCalledWith(
        expect.objectContaining({
          displayName: "Test User",
        }),
      );
    });
  });

  it("handles form input changes", async () => {
    const user = userEvent.setup();
    render(
      <QueryClientProvider client={queryClient}>
        <SettingsView />
      </QueryClientProvider>,
    );

    const displayNameInput = screen.getByDisplayValue("System Administrator");
    await user.clear(displayNameInput);
    await user.type(displayNameInput, "John Doe");

    const emailInput = screen.getByDisplayValue("admin@tracertm.io");
    await user.clear(emailInput);
    await user.type(emailInput, "john@example.com");

    expect(displayNameInput).toHaveValue("John Doe");
    expect(emailInput).toHaveValue("john@example.com");
  });
});
