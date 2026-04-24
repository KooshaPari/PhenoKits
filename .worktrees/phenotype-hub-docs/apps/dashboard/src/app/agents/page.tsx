"use client";

import { motion } from "framer-motion";
import {
  Zap,
  Eye,
  Activity,
  TestTube,
  Clock,
  DollarSign,
  Hash,
  TrendingUp,
  CheckCircle2,
  XCircle,
  AlertCircle,
  Play,
} from "lucide-react";
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  Cell,
} from "recharts";
import { Sidebar } from "@/components/sidebar";
import { DashboardHeader } from "@/components/dashboard-header";
import { agents } from "@/data/agents";

const agentIcons: Record<string, typeof Zap> = {
  forge: Zap,
  muse: Eye,
  sage: Activity,
  helios: TestTube,
};

const taskHistory: Record<string, { task: string; status: string; duration: string }[]> = {
  forge: [
    { task: "Cache adapter TTL logic", status: "in-progress", duration: "2h 15m" },
    { task: "Policy engine rule eval", status: "completed", duration: "3h 42m" },
    { task: "State machine guards", status: "completed", duration: "1h 58m" },
    { task: "Event store hash chain", status: "completed", duration: "4h 10m" },
    { task: "CI pipeline Rust crates", status: "completed", duration: "1h 22m" },
  ],
  muse: [
    { task: "Reviewing PR #127 heliosCLI", status: "in-progress", duration: "45m" },
    { task: "Reviewing PR #126 thegent", status: "completed", duration: "1h 10m" },
    { task: "Code quality audit infrakit", status: "completed", duration: "2h 05m" },
    { task: "Reviewing PR #124 pheno-cli", status: "completed", duration: "55m" },
    { task: "Lint rule updates", status: "completed", duration: "30m" },
  ],
  sage: [
    { task: "NATS streaming research", status: "in-progress", duration: "1h 30m" },
    { task: "LLM provider comparison", status: "completed", duration: "2h 45m" },
    { task: "Agent architecture review", status: "completed", duration: "3h 20m" },
    { task: "Event sourcing patterns", status: "completed", duration: "2h 15m" },
    { task: "Cache strategy analysis", status: "completed", duration: "1h 50m" },
  ],
  helios: [
    { task: "Integration test suite", status: "in-progress", duration: "1h 05m" },
    { task: "Unit test coverage report", status: "completed", duration: "45m" },
    { task: "E2E test pipeline", status: "completed", duration: "2h 30m" },
    { task: "Performance benchmarks", status: "completed", duration: "1h 40m" },
    { task: "FR-003-041 validation", status: "completed", duration: "55m" },
  ],
};

const costBreakdown = [
  { name: "Forge", cost: 118, color: "#7ebab5" },
  { name: "Muse", cost: 48, color: "#8b5cf6" },
  { name: "Sage", cost: 42, color: "#f59e0b" },
  { name: "Helios", cost: 37, color: "#22c55e" },
];

const statusColors: Record<string, string> = {
  "in-progress": "bg-yellow-500/10 text-yellow-400",
  completed: "bg-green-500/10 text-green-400",
  failed: "bg-red-500/10 text-red-400",
};

const statusIcons: Record<string, typeof Play> = {
  "in-progress": Play,
  completed: CheckCircle2,
  failed: XCircle,
};

const tooltipStyle = {
  backgroundColor: "#181b21",
  border: "1px solid rgba(255, 255, 255, 0.1)",
  borderRadius: "8px",
  color: "#f6f5f5",
  fontSize: "12px",
};

export default function AgentsPage() {
  const totalCost = agents.reduce((sum, a) => sum + a.cost, 0);
  const totalSessions = agents.reduce((sum, a) => sum + Math.floor(Math.random() * 5) + 3, 0);

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
              <h1 className="text-2xl font-bold text-white">Agents</h1>
              <p className="mt-1 text-sm text-white/50">
                Active agent fleet — {agents.length} agents · Session total: ${totalCost.toFixed(2)} · {totalSessions} sessions today
              </p>
            </div>

            <div className="mb-6 grid grid-cols-1 gap-6 lg:grid-cols-2">
              {agents.map((agent, i) => {
                const AgentIcon = agentIcons[agent.id];
                const history = taskHistory[agent.id] || [];
                const sessionsToday = Math.floor(Math.random() * 5) + 3;

                return (
                  <motion.div
                    key={agent.id}
                    initial={{ opacity: 0, y: 10 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ delay: i * 0.1, duration: 0.3 }}
                    className="rounded-xl border border-white/10 bg-surface"
                  >
                    <div className="border-b border-white/10 px-6 py-4">
                      <div className="flex items-center gap-3">
                        <div
                          className="flex h-10 w-10 items-center justify-center rounded-full text-sm font-bold"
                          style={{ backgroundColor: `${agent.color}20`, color: agent.color }}
                        >
                          {agent.initial}
                        </div>
                        <div>
                          <div className="flex items-center gap-2">
                            <AgentIcon className="h-4 w-4 text-white/50" />
                            <h3 className="text-lg font-semibold text-white">{agent.name}</h3>
                          </div>
                          <p className="text-xs text-white/50">{agent.role}</p>
                        </div>
                        <span
                          className={`ml-auto inline-flex items-center gap-1.5 rounded-full px-2.5 py-1 text-xs font-medium ${
                            agent.status === "active"
                              ? "bg-green-500/10 text-green-400"
                              : agent.status === "reviewing"
                                ? "bg-yellow-500/10 text-yellow-400"
                                : "bg-gray-500/10 text-gray-400"
                          }`}
                        >
                          <span
                            className={`h-2 w-2 rounded-full ${
                              agent.status === "active"
                                ? "bg-green-500"
                                : agent.status === "reviewing"
                                  ? "bg-yellow-500"
                                  : "bg-gray-500"
                            }`}
                          />
                          {agent.status}
                        </span>
                      </div>
                    </div>

                    <div className="p-6">
                      <div className="mb-4">
                        <div className="flex items-center gap-2 text-sm text-white/70">
                          <TrendingUp className="h-4 w-4" />
                          <span className="font-medium">Current task</span>
                        </div>
                        <p className="mt-1 text-sm text-white">{agent.currentTask}</p>
                        <div className="mt-2">
                          <div className="flex items-center justify-between text-xs">
                            <span className="text-white/50">Progress</span>
                            <span className="font-mono text-white/70">{agent.progress}%</span>
                          </div>
                          <div className="mt-1.5 h-2 overflow-hidden rounded-full bg-white/10">
                            <motion.div
                              initial={{ width: 0 }}
                              animate={{ width: `${agent.progress}%` }}
                              transition={{ duration: 0.8, delay: i * 0.1 + 0.3 }}
                              className="h-full rounded-full"
                              style={{ backgroundColor: agent.color }}
                            />
                          </div>
                        </div>
                      </div>

                      <div className="mb-4 grid grid-cols-3 gap-3">
                        <div className="rounded-lg bg-white/5 p-3 text-center">
                          <DollarSign className="mx-auto h-4 w-4 text-primary" />
                          <div className="mt-1 text-sm font-semibold text-white">${agent.cost.toFixed(2)}</div>
                          <div className="text-xs text-white/40">Session cost</div>
                        </div>
                        <div className="rounded-lg bg-white/5 p-3 text-center">
                          <Hash className="mx-auto h-4 w-4 text-primary" />
                          <div className="mt-1 text-sm font-semibold text-white">{sessionsToday}</div>
                          <div className="text-xs text-white/40">Sessions today</div>
                        </div>
                        <div className="rounded-lg bg-white/5 p-3 text-center">
                          <Clock className="mx-auto h-4 w-4 text-primary" />
                          <div className="mt-1 text-sm font-semibold text-white">{history[0]?.duration}</div>
                          <div className="text-xs text-white/40">Current duration</div>
                        </div>
                      </div>

                      <div>
                        <h4 className="mb-2 text-xs font-semibold uppercase tracking-wider text-white/40">
                          Recent task history
                        </h4>
                        <div className="space-y-2">
                          {history.map((task) => {
                            const StatusIcon = statusIcons[task.status] || Play;
                            return (
                              <div
                                key={task.task}
                                className="flex items-center justify-between rounded-lg bg-white/[0.02] px-3 py-2 text-xs"
                              >
                                <div className="flex items-center gap-2">
                                  <StatusIcon
                                    className={`h-3.5 w-3.5 ${
                                      task.status === "completed"
                                        ? "text-green-400"
                                        : task.status === "in-progress"
                                          ? "text-yellow-400"
                                          : "text-red-400"
                                    }`}
                                  />
                                  <span className="text-white/70">{task.task}</span>
                                </div>
                                <div className="flex items-center gap-2">
                                  <span
                                    className={`rounded-full px-2 py-0.5 text-xs ${statusColors[task.status]}`}
                                  >
                                    {task.status}
                                  </span>
                                  <span className="font-mono text-white/40">{task.duration}</span>
                                </div>
                              </div>
                            );
                          })}
                        </div>
                      </div>
                    </div>
                  </motion.div>
                );
              })}
            </div>

            <div className="rounded-xl border border-white/10 bg-surface p-6">
              <h3 className="mb-4 text-sm font-semibold text-white">LLM Cost Breakdown by Agent (March)</h3>
              <div className="h-52">
                <ResponsiveContainer width="100%" height="100%">
                  <BarChart data={costBreakdown}>
                    <CartesianGrid strokeDasharray="3 3" stroke="rgba(255, 255, 255, 0.06)" />
                    <XAxis
                      dataKey="name"
                      stroke="rgba(255, 255, 255, 0.3)"
                      fontSize={11}
                      tickLine={false}
                      axisLine={false}
                    />
                    <YAxis
                      stroke="rgba(255, 255, 255, 0.3)"
                      fontSize={11}
                      tickLine={false}
                      axisLine={false}
                      tickFormatter={(v: number) => `$${v}`}
                    />
                    <Tooltip
                      contentStyle={tooltipStyle}
                      formatter={(value: number) => [`$${value.toFixed(2)}`, "Cost"]}
                    />
                    <Bar dataKey="cost" radius={[4, 4, 0, 0]}>
                      {costBreakdown.map((entry) => (
                        <Cell key={entry.name} fill={entry.color} />
                      ))}
                    </Bar>
                  </BarChart>
                </ResponsiveContainer>
              </div>
            </div>
          </motion.div>
        </main>
      </div>
    </div>
  );
}
