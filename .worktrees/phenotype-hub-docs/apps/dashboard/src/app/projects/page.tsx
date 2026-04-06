"use client";

import { useState, useMemo } from "react";
import Link from "next/link";
import { motion } from "framer-motion";
import {
  ArrowUpDown,
  ArrowUp,
  ArrowDown,
  Search,
  Filter,
  GitBranch,
  GitCommit,
  CheckCircle2,
  XCircle,
  AlertCircle,
  ExternalLink,
} from "lucide-react";
import { Sidebar } from "@/components/sidebar";
import { DashboardHeader } from "@/components/dashboard-header";
import { projects } from "@/data/projects";

type SortKey = "name" | "status" | "category" | "lastCommit" | "ciStatus" | "branch";
type SortDir = "asc" | "desc";

const statusColors: Record<string, string> = {
  healthy: "bg-green-500/10 text-green-400 border-green-500/20",
  degraded: "bg-yellow-500/10 text-yellow-400 border-yellow-500/20",
  down: "bg-red-500/10 text-red-400 border-red-500/20",
};

const ciIcons: Record<string, typeof CheckCircle2> = {
  passing: CheckCircle2,
  failing: XCircle,
  pending: AlertCircle,
};

const ciColors: Record<string, string> = {
  passing: "text-green-400",
  failing: "text-red-400",
  pending: "text-yellow-400",
};

const categoryLabels: Record<string, string> = {
  platform: "Platform",
  agent: "Agent",
  api: "API",
  lib: "Library",
  tool: "Tool",
  app: "App",
};

function formatRelativeTime(dateStr: string): string {
  const date = new Date(dateStr);
  const now = new Date("2026-03-31T12:00:00Z");
  const diffMs = now.getTime() - date.getTime();
  const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
  const diffDays = Math.floor(diffHours / 24);

  if (diffHours < 1) return "just now";
  if (diffHours < 24) return `${diffHours}h ago`;
  if (diffDays === 1) return "yesterday";
  if (diffDays < 7) return `${diffDays}d ago`;
  return date.toLocaleDateString("en-US", { month: "short", day: "numeric" });
}

export default function ProjectsPage() {
  const [search, setSearch] = useState("");
  const [statusFilter, setStatusFilter] = useState<string>("all");
  const [categoryFilter, setCategoryFilter] = useState<string>("all");
  const [sortKey, setSortKey] = useState<SortKey>("name");
  const [sortDir, setSortDir] = useState<SortDir>("asc");

  const handleSort = (key: SortKey) => {
    if (sortKey === key) {
      setSortDir((d) => (d === "asc" ? "desc" : "asc"));
    } else {
      setSortKey(key);
      setSortDir("asc");
    }
  };

  const SortIcon = ({ column }: { column: SortKey }) => {
    if (sortKey !== column) return <ArrowUpDown className="ml-1 h-3.5 w-3.5 text-white/30" />;
    return sortDir === "asc" ? (
      <ArrowUp className="ml-1 h-3.5 w-3.5 text-primary" />
    ) : (
      <ArrowDown className="ml-1 h-3.5 w-3.5 text-primary" />
    );
  };

  const filtered = useMemo(() => {
    let result = [...projects];

    if (search) {
      const q = search.toLowerCase();
      result = result.filter(
        (p) =>
          p.name.toLowerCase().includes(q) ||
          p.description.toLowerCase().includes(q)
      );
    }

    if (statusFilter !== "all") {
      result = result.filter((p) => p.status === statusFilter);
    }

    if (categoryFilter !== "all") {
      result = result.filter((p) => p.category === categoryFilter);
    }

    result.sort((a, b) => {
      const aVal = a[sortKey];
      const bVal = b[sortKey];
      const cmp = aVal.localeCompare(bVal);
      return sortDir === "asc" ? cmp : -cmp;
    });

    return result;
  }, [search, statusFilter, categoryFilter, sortKey, sortDir]);

  const categories = [...new Set(projects.map((p) => p.category))].sort();

  return (
    <div className="min-h-screen bg-background">
      <Sidebar />

      <div className="pl-60">
        <DashboardHeader />

        <main className="p-6">
          <motion.div
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.3 }}
          >
            <div className="mb-6">
              <h1 className="text-2xl font-bold text-white">Projects</h1>
              <p className="mt-1 text-sm text-white/50">
                All {projects.length} projects in the ecosystem
              </p>
            </div>

            <div className="mb-6 flex flex-wrap items-center gap-3">
              <div className="relative">
                <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-white/40" />
                <input
                  type="text"
                  placeholder="Search projects..."
                  value={search}
                  onChange={(e) => setSearch(e.target.value)}
                  className="w-64 rounded-lg border border-white/10 bg-surface py-2 pl-10 pr-4 text-sm text-white placeholder:text-white/40 focus:border-primary/50 focus:outline-none focus:ring-1 focus:ring-primary/20"
                />
              </div>

              <div className="flex items-center gap-2">
                <Filter className="h-4 w-4 text-white/40" />
                <select
                  value={statusFilter}
                  onChange={(e) => setStatusFilter(e.target.value)}
                  className="rounded-lg border border-white/10 bg-surface px-3 py-2 text-sm text-white focus:border-primary/50 focus:outline-none focus:ring-1 focus:ring-primary/20"
                >
                  <option value="all">All Status</option>
                  <option value="healthy">Healthy</option>
                  <option value="degraded">Degraded</option>
                  <option value="down">Down</option>
                </select>

                <select
                  value={categoryFilter}
                  onChange={(e) => setCategoryFilter(e.target.value)}
                  className="rounded-lg border border-white/10 bg-surface px-3 py-2 text-sm text-white focus:border-primary/50 focus:outline-none focus:ring-1 focus:ring-primary/20"
                >
                  <option value="all">All Types</option>
                  {categories.map((c) => (
                    <option key={c} value={c}>
                      {categoryLabels[c]}
                    </option>
                  ))}
                </select>
              </div>

              <span className="ml-auto text-xs text-white/40">
                {filtered.length} of {projects.length} projects
              </span>
            </div>

            <div className="overflow-hidden rounded-xl border border-white/10 bg-surface">
              <div className="overflow-x-auto">
                <table className="w-full text-left text-sm">
                  <thead>
                    <tr className="border-b border-white/10">
                      <th className="px-4 py-3 text-xs font-semibold uppercase tracking-wider text-white/50">
                        <button
                          type="button"
                          onClick={() => handleSort("name")}
                          className="flex items-center text-xs font-semibold uppercase tracking-wider text-white/50 hover:text-white/70"
                        >
                          Name
                          <SortIcon column="name" />
                        </button>
                      </th>
                      <th className="px-4 py-3 text-xs font-semibold uppercase tracking-wider text-white/50">
                        <button
                          type="button"
                          onClick={() => handleSort("status")}
                          className="flex items-center text-xs font-semibold uppercase tracking-wider text-white/50 hover:text-white/70"
                        >
                          Status
                          <SortIcon column="status" />
                        </button>
                      </th>
                      <th className="px-4 py-3 text-xs font-semibold uppercase tracking-wider text-white/50">
                        <button
                          type="button"
                          onClick={() => handleSort("category")}
                          className="flex items-center text-xs font-semibold uppercase tracking-wider text-white/50 hover:text-white/70"
                        >
                          Type
                          <SortIcon column="category" />
                        </button>
                      </th>
                      <th className="px-4 py-3 text-xs font-semibold uppercase tracking-wider text-white/50">
                        <button
                          type="button"
                          onClick={() => handleSort("lastCommit")}
                          className="flex items-center text-xs font-semibold uppercase tracking-wider text-white/50 hover:text-white/70"
                        >
                          Last Commit
                          <SortIcon column="lastCommit" />
                        </button>
                      </th>
                      <th className="px-4 py-3 text-xs font-semibold uppercase tracking-wider text-white/50">
                        <button
                          type="button"
                          onClick={() => handleSort("ciStatus")}
                          className="flex items-center text-xs font-semibold uppercase tracking-wider text-white/50 hover:text-white/70"
                        >
                          CI
                          <SortIcon column="ciStatus" />
                        </button>
                      </th>
                      <th className="px-4 py-3 text-xs font-semibold uppercase tracking-wider text-white/50">
                        <button
                          type="button"
                          onClick={() => handleSort("branch")}
                          className="flex items-center text-xs font-semibold uppercase tracking-wider text-white/50 hover:text-white/70"
                        >
                          Branch
                          <SortIcon column="branch" />
                        </button>
                      </th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-white/5">
                    {filtered.map((project, i) => {
                      const CIIcon = ciIcons[project.ciStatus];
                      return (
                        <motion.tr
                          key={project.name}
                          initial={{ opacity: 0, y: 5 }}
                          animate={{ opacity: 1, y: 0 }}
                          transition={{ delay: i * 0.015, duration: 0.2 }}
                          className="transition-colors hover:bg-white/[0.02]"
                        >
                          <td className="px-4 py-3">
                            <div className="flex items-center gap-2">
                              <span className="font-medium text-white">{project.name}</span>
                              <ExternalLink className="h-3 w-3 text-white/20" />
                            </div>
                          </td>
                          <td className="px-4 py-3">
                            <span
                              className={`inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-medium ${statusColors[project.status]}`}
                            >
                              {project.status}
                            </span>
                          </td>
                          <td className="px-4 py-3">
                            <span className="rounded-md bg-white/5 px-2 py-0.5 text-xs text-white/60">
                              {categoryLabels[project.category]}
                            </span>
                          </td>
                          <td className="px-4 py-3">
                            <div className="flex items-center gap-1.5 text-xs text-white/50">
                              <GitCommit className="h-3.5 w-3.5" />
                              {formatRelativeTime(project.lastCommit)}
                            </div>
                          </td>
                          <td className="px-4 py-3">
                            <div className={`flex items-center gap-1 text-xs ${ciColors[project.ciStatus]}`}>
                              <CIIcon className="h-3.5 w-3.5" />
                              {project.ciStatus}
                            </div>
                          </td>
                          <td className="px-4 py-3">
                            <div className="flex items-center gap-1.5 text-xs font-mono text-white/50">
                              <GitBranch className="h-3.5 w-3.5" />
                              {project.branch}
                            </div>
                          </td>
                        </motion.tr>
                      );
                    })}
                  </tbody>
                </table>
              </div>
            </div>
          </motion.div>
        </main>
      </div>
    </div>
  );
}
