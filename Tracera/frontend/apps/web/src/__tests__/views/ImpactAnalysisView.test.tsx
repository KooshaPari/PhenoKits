/**
 * Comprehensive Tests for ImpactAnalysisView
 * Traces to: FR-TRACERA-VIEWS-002
 */

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";

import { useItems } from "../../hooks/useItems";
import { useLinks } from "../../hooks/useLinks";
import { ImpactAnalysisView } from "../../views/ImpactAnalysisView";

vi.mock("../../hooks/useItems", () => ({
  useItems: vi.fn(),
}));

vi.mock("../../hooks/useLinks", () => ({
  useLinks: vi.fn(),
}));

describe(ImpactAnalysisView, () => {
  let queryClient: QueryClient;

  beforeEach(() => {
    queryClient = new QueryClient({
      defaultOptions: {
        mutations: { retry: false },
        queries: { retry: false },
      },
    });
    vi.clearAllMocks();
  });

  it("renders impact analysis interface", () => {
    vi.mocked(useItems).mockReturnValue({
      data: [],
      error: null,
      isError: false,
      isLoading: false,
    } as any);

    vi.mocked(useLinks).mockReturnValue({
      data: [],
      error: null,
      isError: false,
      isLoading: false,
    } as any);

    render(
      <QueryClientProvider client={queryClient}>
        <ImpactAnalysisView projectId="proj-test" />
      </QueryClientProvider>,
    );

    expect(screen.getByText("Impact Intelligence")).toBeInTheDocument();
    expect(screen.getByText("System Standby")).toBeInTheDocument();
  });

  it("displays impact analysis results", async () => {
    const items = [
      { id: "item-1", title: "Test Item", type: "Feature", view: "Feature" },
      {
        id: "item-2",
        title: "Affected Item 1",
        type: "Story",
        view: "Feature",
      },
      {
        id: "item-3",
        title: "Affected Item 2",
        type: "Story",
        view: "Feature",
      },
    ];

    const links = [
      {
        id: "link-1",
        sourceId: "item-1",
        targetId: "item-2",
        type: "implements",
      },
      {
        id: "link-2",
        sourceId: "item-1",
        targetId: "item-3",
        type: "implements",
      },
    ];

    vi.mocked(useItems).mockReturnValue({
      data: { items },
      error: null,
      isError: false,
      isLoading: false,
    } as any);

    vi.mocked(useLinks).mockReturnValue({
      data: { links },
      error: null,
      isError: false,
      isLoading: false,
    } as any);

    render(
      <QueryClientProvider client={queryClient}>
        <ImpactAnalysisView projectId="proj-test" />
      </QueryClientProvider>,
    );

    await userEvent.click(screen.getByText("Test Item"));

    await waitFor(() => {
      expect(screen.getByText("Analysis Report")).toBeInTheDocument();
      expect(screen.getAllByText("Affected Item 1").length).toBeGreaterThan(0);
      expect(screen.getAllByText("Affected Item 2").length).toBeGreaterThan(0);
    });
  });
});
