"use client";

import { useState, useMemo } from "react";
import { motion } from "framer-motion";
import {
  FileText,
  Table2,
  Kanban,
  Search,
  Filter,
  Zap,
  Eye,
  Activity,
  TestTube,
} from "lucide-react";
import { Sidebar } from "@/components/sidebar";
import { DashboardHeader } from "@/components/dashboard-header";
import { SpecPipeline } from "@/components/spec-pipeline";
import { specsList, type SpecEntry } from "@/data/specs-list";
import { stages } from "@/data/specs";

const statusColors: Record<string, string> = {
  triage: "bg-gray-500/10 text-gray-400 border-gray-500/20",
  specified: "bg-blue-500/10 text-blue-400 border-blue-500/20",
  inProgress: "bg-purple-500/10 text-purple-400 border-purple-500/20",
  validating: "bg-yellow-500/10 text-yellow-400 border-yellow-500/20",
  shipped: "bg-green-500/10 text-green-400 border-green-500/20",
};

const agentIcons: Record<string, typeof Zap> = {
  Forge: Zap,
  Muse: Eye,
  Sage: Activity,
  Helios: TestTube,
};

const agentColors: Record<string, string> = {
  Forge: "#7ebab5",
  Muse: "#8b5cf6",
  Sage: "#f59e0b",
  Helios: "#22c55e",
};

const kanbanOrder = ["triage", "specified", "inProgress", "validating", "shipped"];

export default function SpecsPage() {
  const [view, setView] = useState<"table" | "kanban">("table");
  const [search, setSearch] = useState("");
  const [statusFilter, setStatusFilter] = useState<string>("all");

  const filtered = useMemo(() => {
    let result = [...specsList];

    if (search) {
      const q = search.toLowerCase();
      result = result.filter(
        (s) =>
          s.title.toLowerCase().includes(q) ||
          s.id.toLowerCase().includes(q)
      );
    }

    if (statusFilter !== "all") {
      result = result.filter((s) => s.status === statusFilter);
    }

    return result;
  }, [search, statusFilter]);

  const kanbanData = useMemo(() => {
    const columns: Record<string, SpecEntry[]> = {};
    kanbanOrder.forEach((key) => {
      columns[key] = filtered.filter((s) => s.status === key);
    });
    return columns;
  }, [filtered]);

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
              <h1 className="text-2xl font-bold text-white">Spec Pipeline</h1>
              <p className="mt-1 text-sm text-white/50">
                Track specifications from triage through delivery
              </p>
            </div>

            <div className="mb-6">
              <SpecPipeline />
            </div>

            <div className="mb-6 flex flex-wrap items-center gap-3">
              <div className="relative">
                <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-white/40" />
                <input
                  type="text"
                  placeholder="Search specs..."
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
                  <option value="all">All Stages</option>
                  {stages.map((s) => (
                    <option key={s.key} value={s.key}>
                      {s.label}
                    </option>
                  ))}
                </select>
              </div>

              <div className="ml-auto flex items-center gap-1 rounded-lg border border-white/10 bg-surface p-1">
                <button
                  type="button"
                  onClick={() => setView("table")}
                  className={`flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs font-medium transition-colors ${
                    view === "table"
                      ? "bg-primary/10 text-primary"
                      : "text-white/50 hover:text-white/70"
                  }`}
                >
                  <Table2 className="h-3.5 w-3.5" />
                  Table
                </button>
                <button
                  type="button"
                  onClick={() => setView("kanban")}
                  className={`flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs font-medium transition-colors ${
                    view === "kanban"
                      ? "bg-primary/10 text-primary"
                      : "text-white/50 hover:text-white/70"
                  }`}
                >
                  <Kanban className="h-3.5 w-3.5" />
                  Kanban
                </button>
              </div>

              <span className="text-xs text-white/40">
                {filtered.length} specs
              </span>
            </div>

            {view === "table" ? (
              <div className="overflow-hidden rounded-xl border border-white/10 bg-surface">
                <div className="overflow-x-auto">
                  <table className="w-full text-left text-sm">
                    <thead>
                      <tr className="border-b border-white/10">
                        <th className="px-4 py-3 text-xs font-semibold uppercase tracking-wider text-white/50">
                          ID
                        </th>
                        <th className="px-4 py-3 text-xs font-semibold uppercase tracking-wider text-white/50">
                          Title
                        </th>
                        <th className="px-4 py-3 text-xs font-semibold uppercase tracking-wider text-white/50">
                          Stage
                        </th>
                        <th className="px-4 py-3 text-xs font-semibold uppercase tracking-wider text-white/50">
                          Agent
                        </th>
                        <th className="px-4 py-3 text-xs font-semibold uppercase tracking-wider text-white/50">
                          Created
                        </th>
                      </tr>
                    </thead>
                    <tbody className="divide-y divide-white/5">
                      {filtered.map((spec, i) => {
                        const AgentIcon = agentIcons[spec.agent];
                        return (
                          <motion.tr
                            key={spec.id}
                            initial={{ opacity: 0, y: 5 }}
                            animate={{ opacity: 1, y: 0 }}
                            transition={{ delay: i * 0.02, duration: 0.2 }}
                            className="transition-colors hover:bg-white/[0.02]"
                          >
                            <td className="px-4 py-3">
                              <span className="font-mono text-xs text-primary">{spec.id}</span>
                            </td>
                            <td className="px-4 py-3">
                              <span className="text-white/80">{spec.title}</span>
                            </td>
                            <td className="px-4 py-3">
                              <span
                                className={`inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-medium ${statusColors[spec.status]}`}
                              >
                                {stages.find((s) => s.key === spec.status)?.label || spec.status}
                              </span>
                            </td>
                            <td className="px-4 py-3">
                              <div className="flex items-center gap-2">
                                <AgentIcon
                                  className="h-3.5 w-3.5"
                                  style={{ color: agentColors[spec.agent] }}
                                />
                                <span className="text-xs text-white/60">{spec.agent}</span>
                              </div>
                            </td>
                            <td className="px-4 py-3">
                              <span className="text-xs text-white/40">{spec.created}</span>
                            </td>
                          </motion.tr>
                        );
                      })}
                    </tbody>
                  </table>
                </div>
              </div>
            ) : (
              <div className="flex gap-4 overflow-x-auto">
                {kanbanOrder.map((key) => {
                  const column = kanbanData[key] || [];
                  const stage = stages.find((s) => s.key === key);
                  return (
                    <div key={key} className="min-w-[260px] flex-1">
                      <div className="mb-3 flex items-center justify-between">
                        <h3 className="text-sm font-semibold text-white">{stage?.label}</h3>
                        <span className="rounded-full bg-white/10 px-2 py-0.5 text-xs text-white/50">
                          {column.length}
                        </span>
                      </div>
                      <div className="space-y-2">
                        {column.map((spec) => {
                          const AgentIcon = agentIcons[spec.agent];
                          return (
                            <motion.div
                              key={spec.id}
                              initial={{ opacity: 0, y: 5 }}
                              animate={{ opacity: 1, y: 0 }}
                              transition={{ duration: 0.2 }}
                              className="rounded-lg border border-white/10 bg-surface p-3 transition-colors hover:border-white/20"
                            >
                              <div className="mb-2 flex items-start justify-between">
                                <span className="font-mono text-xs text-primary">{spec.id}</span>
                                <AgentIcon
                                  className="h-3.5 w-3.5"
                                  style={{ color: agentColors[spec.agent] }}
                                />
                              </div>
                              <p className="text-xs text-white/70">{spec.title}</p>
                              <p className="mt-2 text-xs text-white/30">{spec.created}</p>
                            </motion.div>
                          );
                        })}
                      </div>
                    </div>
                  );
                })}
              </div>
            )}
          </motion.div>
        </main>
      </div>
    </div>
  );
}
