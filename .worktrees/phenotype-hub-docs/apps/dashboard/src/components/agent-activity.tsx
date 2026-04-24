"use client";

import { motion } from "framer-motion";
import { Activity, Zap, Eye, TestTube } from "lucide-react";
import { agents, type Agent } from "@/data/agents";

const statusColors: Record<string, string> = {
  active: "bg-green-500",
  idle: "bg-gray-500",
  reviewing: "bg-yellow-500",
};

const agentIcons = {
  forge: Zap,
  muse: Eye,
  sage: Activity,
  helios: TestTube,
};

function AgentCard({ agent, index }: { agent: Agent; index: number }) {
  const AgentIcon = agentIcons[agent.id as keyof typeof agentIcons];

  return (
    <motion.div
      initial={{ opacity: 0, x: -10 }}
      animate={{ opacity: 1, x: 0 }}
      transition={{ delay: index * 0.1, duration: 0.3 }}
      whileHover={{ x: 4 }}
      className="rounded-lg border border-white/10 bg-surface-elevated p-4 transition-colors"
    >
      <div className="flex items-start gap-3">
        <div
          className="flex h-10 w-10 shrink-0 items-center justify-center rounded-full text-sm font-bold"
          style={{ backgroundColor: `${agent.color}20`, color: agent.color }}
        >
          {agent.initial}
        </div>

        <div className="min-w-0 flex-1">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <AgentIcon className="h-4 w-4 text-white/50" />
              <span className="text-sm font-semibold text-white">{agent.name}</span>
            </div>
            <span
              className={`inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-xs font-medium ${
                agent.status === "active"
                  ? "bg-green-500/10 text-green-400"
                  : agent.status === "reviewing"
                    ? "bg-yellow-500/10 text-yellow-400"
                    : "bg-gray-500/10 text-gray-400"
              }`}
            >
              <span className={`h-1.5 w-1.5 rounded-full ${statusColors[agent.status]}`} />
              {agent.status}
            </span>
          </div>

          <p className="mt-1 truncate text-xs text-white/60">{agent.role}</p>
          <p className="mt-2 truncate text-sm text-white/80">{agent.currentTask}</p>

          <div className="mt-3">
            <div className="flex items-center justify-between text-xs">
              <span className="text-white/50">Progress</span>
              <span className="font-mono text-white/70">{agent.progress}%</span>
            </div>
            <div className="mt-1.5 h-1.5 overflow-hidden rounded-full bg-white/10">
              <motion.div
                initial={{ width: 0 }}
                animate={{ width: `${agent.progress}%` }}
                transition={{ duration: 0.8, delay: index * 0.1 + 0.3 }}
                className="h-full rounded-full"
                style={{ backgroundColor: agent.color }}
              />
            </div>
          </div>

          <div className="mt-3 flex items-center justify-between text-xs">
            <span className="text-white/40">Session cost</span>
            <span className="font-mono text-primary">${agent.cost.toFixed(2)}</span>
          </div>
        </div>
      </div>
    </motion.div>
  );
}

export function AgentActivity() {
  const totalCost = agents.reduce((sum, a) => sum + a.cost, 0);

  return (
    <div className="rounded-xl border border-white/10 bg-surface">
      <div className="flex items-center justify-between border-b border-white/10 px-6 py-4">
        <h2 className="text-lg font-semibold text-white">Agent Activity</h2>
        <span className="text-xs text-white/50">
          Session total: <span className="font-mono text-primary">${totalCost.toFixed(2)}</span>
        </span>
      </div>

      <div className="grid grid-cols-1 gap-4 p-6 lg:grid-cols-2">
        {agents.map((agent, i) => (
          <AgentCard key={agent.id} agent={agent} index={i} />
        ))}
      </div>
    </div>
  );
}
