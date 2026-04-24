"use client";

import { motion } from "framer-motion";
import {
  Clock,
  AlertTriangle,
  DollarSign,
  Rocket,
  FileText,
  CheckCircle2,
  GitCommit,
} from "lucide-react";
import { Sidebar } from "@/components/sidebar";
import { DashboardHeader } from "@/components/dashboard-header";
import { ProjectHealth } from "@/components/project-health";
import { AgentActivity } from "@/components/agent-activity";
import { SpecPipeline } from "@/components/spec-pipeline";
import {
  ReleaseVelocityChart,
  LLmCostChart,
  CycleTimeChart,
} from "@/components/metrics-chart";

const statCards = [
  {
    label: "Releases This Month",
    value: "22",
    change: "+18%",
    icon: Rocket,
    color: "text-primary",
    bg: "bg-primary/10",
  },
  {
    label: "Avg Cycle Time",
    value: "24h",
    change: "-14%",
    icon: Clock,
    color: "text-green-400",
    bg: "bg-green-500/10",
  },
  {
    label: "Open Issues",
    value: "16",
    change: "-3",
    icon: AlertTriangle,
    color: "text-yellow-400",
    bg: "bg-yellow-500/10",
  },
  {
    label: "LLM Cost (Mar)",
    value: "$245",
    change: "-8%",
    icon: DollarSign,
    color: "text-purple-400",
    bg: "bg-purple-500/10",
  },
];

const recentActivity = [
  {
    id: 1,
    type: "spec",
    description: "Spec FR-004-018 created: NATS event streaming integration",
    time: "2 hours ago",
    icon: FileText,
    color: "text-blue-400",
  },
  {
    id: 2,
    type: "pr",
    description: "PR #127 merged on heliosCLI: token refresh with backoff",
    time: "4 hours ago",
    icon: GitCommit,
    color: "text-green-400",
  },
  {
    id: 3,
    type: "release",
    description: "Release v2.4.0 published for phenotype-infrakit",
    time: "6 hours ago",
    icon: Rocket,
    color: "text-primary",
  },
  {
    id: 4,
    type: "validation",
    description: "FR-003-041 validated: policy engine TOML parser",
    time: "8 hours ago",
    icon: CheckCircle2,
    color: "text-green-400",
  },
  {
    id: 5,
    type: "spec",
    description: "Spec FR-005-002 triaged: health check aggregation",
    time: "12 hours ago",
    icon: FileText,
    color: "text-blue-400",
  },
  {
    id: 6,
    type: "pr",
    description: "PR #126 merged on thegent: governance rule updates",
    time: "1 day ago",
    icon: GitCommit,
    color: "text-green-400",
  },
];

export default function DashboardPage() {
  return (
    <div className="min-h-screen bg-background">
      <Sidebar />

      <div className="pl-60">
        <DashboardHeader />

        <main className="p-6">
          <div className="mb-6">
            <h1 className="text-2xl font-bold text-white">Overview</h1>
            <p className="mt-1 text-sm text-white/50">
              Real-time health metrics for your engineering organization
            </p>
          </div>

          <div className="mb-6 grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
            {statCards.map((stat, i) => (
              <motion.div
                key={stat.label}
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: i * 0.05, duration: 0.3 }}
                whileHover={{ y: -2 }}
                className="rounded-xl border border-white/10 bg-surface p-5"
              >
                <div className="flex items-center justify-between">
                  <div className={`rounded-lg ${stat.bg} p-2.5`}>
                    <stat.icon className={`h-5 w-5 ${stat.color}`} />
                  </div>
                  <span
                    className={`text-xs font-medium ${
                      stat.change.startsWith("+")
                        ? stat.change.startsWith("-")
                          ? "text-green-400"
                          : "text-green-400"
                        : "text-green-400"
                    }`}
                  >
                    {stat.change}
                  </span>
                </div>
                <div className="mt-3">
                  <div className="text-2xl font-bold text-white">{stat.value}</div>
                  <div className="text-xs text-white/50">{stat.label}</div>
                </div>
              </motion.div>
            ))}
          </div>

          <div className="mb-6">
            <ProjectHealth />
          </div>

          <div className="mb-6 grid grid-cols-1 gap-6 lg:grid-cols-3">
            <div className="lg:col-span-2">
              <AgentActivity />
            </div>
            <SpecPipeline />
          </div>

          <div className="mb-6 grid grid-cols-1 gap-6 lg:grid-cols-3">
            <div className="rounded-xl border border-white/10 bg-surface p-6">
              <h3 className="mb-4 text-sm font-semibold text-white">Release Velocity</h3>
              <ReleaseVelocityChart />
            </div>
            <div className="rounded-xl border border-white/10 bg-surface p-6">
              <h3 className="mb-4 text-sm font-semibold text-white">LLM Cost by Provider</h3>
              <LLmCostChart />
            </div>
            <div className="rounded-xl border border-white/10 bg-surface p-6">
              <h3 className="mb-4 text-sm font-semibold text-white">Cycle Time Trend</h3>
              <CycleTimeChart />
            </div>
          </div>

          <div className="rounded-xl border border-white/10 bg-surface">
            <div className="border-b border-white/10 px-6 py-4">
              <h2 className="text-lg font-semibold text-white">Recent Activity</h2>
            </div>
            <div className="divide-y divide-white/5">
              {recentActivity.map((item, i) => (
                <motion.div
                  key={item.id}
                  initial={{ opacity: 0, x: -10 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: i * 0.05, duration: 0.3 }}
                  className="flex items-start gap-4 px-6 py-4"
                >
                  <div className="mt-0.5 rounded-full bg-white/5 p-2">
                    <item.icon className={`h-4 w-4 ${item.color}`} />
                  </div>
                  <div className="flex-1">
                    <p className="text-sm text-white/80">{item.description}</p>
                    <p className="mt-0.5 text-xs text-white/40">{item.time}</p>
                  </div>
                </motion.div>
              ))}
            </div>
          </div>
        </main>
      </div>
    </div>
  );
}
