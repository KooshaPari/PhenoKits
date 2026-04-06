"use client";

import { motion } from "framer-motion";
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  LineChart,
  Line,
  AreaChart,
  Area,
  PieChart,
  Pie,
  Cell,
} from "recharts";
import { Sidebar } from "@/components/sidebar";
import { DashboardHeader } from "@/components/dashboard-header";
import {
  releaseVelocity,
  cycleTimeTrend,
  llmCostByProvider,
  llmCostByAgent,
  projectHealthOverTime,
} from "@/data/metrics";

const tooltipStyle = {
  backgroundColor: "#181b21",
  border: "1px solid rgba(255, 255, 255, 0.1)",
  borderRadius: "8px",
  color: "#f6f5f5",
  fontSize: "12px",
};

const providerColors = {
  claude: "#d97706",
  gemini: "#3b82f6",
  openai: "#22c55e",
};

const agentColors = {
  forge: "#7ebab5",
  muse: "#8b5cf6",
  sage: "#f59e0b",
  helios: "#22c55e",
};

const healthColors = {
  healthy: "#22c55e",
  degraded: "#f59e0b",
  down: "#ef4444",
};

export default function MetricsPage() {
  const totalReleases = releaseVelocity.reduce((sum, d) => sum + d.releases, 0);
  const avgCycleTime = Math.round(
    cycleTimeTrend.reduce((sum, d) => sum + d.hours, 0) / cycleTimeTrend.length
  );
  const latestCost = llmCostByProvider[llmCostByProvider.length - 1];
  const totalCost = llmCostByProvider.reduce((sum, d) => sum + d.total, 0);

  const pieData = [
    { name: "Claude", value: latestCost.claude, color: providerColors.claude },
    { name: "Gemini", value: latestCost.gemini, color: providerColors.gemini },
    { name: "OpenAI", value: latestCost.openai, color: providerColors.openai },
  ];

  const agentPieData = [
    { name: "Forge", value: llmCostByAgent[llmCostByAgent.length - 1].forge, color: agentColors.forge },
    { name: "Muse", value: llmCostByAgent[llmCostByAgent.length - 1].muse, color: agentColors.muse },
    { name: "Sage", value: llmCostByAgent[llmCostByAgent.length - 1].sage, color: agentColors.sage },
    { name: "Helios", value: llmCostByAgent[llmCostByAgent.length - 1].helios, color: agentColors.helios },
  ];

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
              <h1 className="text-2xl font-bold text-white">Metrics</h1>
              <p className="mt-1 text-sm text-white/50">
                Deep-dive into engineering performance — {totalReleases} releases · {avgCycleTime}h avg cycle · ${totalCost.toFixed(0)} total LLM spend
              </p>
            </div>

            <div className="mb-6 grid grid-cols-1 gap-6 lg:grid-cols-2">
              <div className="rounded-xl border border-white/10 bg-surface p-6">
                <h3 className="mb-4 text-sm font-semibold text-white">Release Velocity (12 months)</h3>
                <div className="h-64">
                  <ResponsiveContainer width="100%" height="100%">
                    <BarChart data={releaseVelocity}>
                      <CartesianGrid strokeDasharray="3 3" stroke="rgba(255, 255, 255, 0.06)" />
                      <XAxis
                        dataKey="month"
                        stroke="rgba(255, 255, 255, 0.3)"
                        fontSize={10}
                        tickLine={false}
                        axisLine={false}
                      />
                      <YAxis
                        stroke="rgba(255, 255, 255, 0.3)"
                        fontSize={11}
                        tickLine={false}
                        axisLine={false}
                      />
                      <Tooltip contentStyle={tooltipStyle} />
                      <Bar dataKey="releases" fill="#7ebab5" radius={[4, 4, 0, 0]} />
                    </BarChart>
                  </ResponsiveContainer>
                </div>
              </div>

              <div className="rounded-xl border border-white/10 bg-surface p-6">
                <h3 className="mb-4 text-sm font-semibold text-white">Cycle Time Trend (12 months)</h3>
                <div className="h-64">
                  <ResponsiveContainer width="100%" height="100%">
                    <AreaChart data={cycleTimeTrend}>
                      <defs>
                        <linearGradient id="cycleGrad" x1="0" y1="0" x2="0" y2="1">
                          <stop offset="5%" stopColor="#7ebab5" stopOpacity={0.3} />
                          <stop offset="95%" stopColor="#7ebab5" stopOpacity={0} />
                        </linearGradient>
                      </defs>
                      <CartesianGrid strokeDasharray="3 3" stroke="rgba(255, 255, 255, 0.06)" />
                      <XAxis
                        dataKey="month"
                        stroke="rgba(255, 255, 255, 0.3)"
                        fontSize={10}
                        tickLine={false}
                        axisLine={false}
                      />
                      <YAxis
                        stroke="rgba(255, 255, 255, 0.3)"
                        fontSize={11}
                        tickLine={false}
                        axisLine={false}
                        tickFormatter={(v: number) => `${v}h`}
                      />
                      <Tooltip
                        contentStyle={tooltipStyle}
                        formatter={(value: number) => [`${value}h`, "Cycle Time"]}
                      />
                      <Area
                        type="monotone"
                        dataKey="hours"
                        stroke="#7ebab5"
                        strokeWidth={2}
                        fill="url(#cycleGrad)"
                        dot={{ fill: "#7ebab5", strokeWidth: 2, r: 3 }}
                        activeDot={{ r: 5 }}
                      />
                    </AreaChart>
                  </ResponsiveContainer>
                </div>
              </div>
            </div>

            <div className="mb-6 grid grid-cols-1 gap-6 lg:grid-cols-2">
              <div className="rounded-xl border border-white/10 bg-surface p-6">
                <h3 className="mb-4 text-sm font-semibold text-white">LLM Cost by Provider (12 months)</h3>
                <div className="h-64">
                  <ResponsiveContainer width="100%" height="100%">
                    <BarChart data={llmCostByProvider}>
                      <CartesianGrid strokeDasharray="3 3" stroke="rgba(255, 255, 255, 0.06)" />
                      <XAxis
                        dataKey="month"
                        stroke="rgba(255, 255, 255, 0.3)"
                        fontSize={10}
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
                        formatter={(value: number) => [`$${value.toFixed(2)}`, ""]}
                      />
                      <Bar dataKey="claude" stackId="a" fill={providerColors.claude} radius={[0, 0, 0, 0]} />
                      <Bar dataKey="gemini" stackId="a" fill={providerColors.gemini} />
                      <Bar dataKey="openai" stackId="a" fill={providerColors.openai} radius={[4, 4, 0, 0]} />
                    </BarChart>
                  </ResponsiveContainer>
                </div>
                <div className="mt-3 flex justify-center gap-4 text-xs">
                  {pieData.map((item) => (
                    <span key={item.name} className="flex items-center gap-1.5 text-white/60">
                      <span className="h-2 w-2 rounded-full" style={{ backgroundColor: item.color }} />
                      {item.name} (${item.value.toFixed(2)})
                    </span>
                  ))}
                </div>
              </div>

              <div className="rounded-xl border border-white/10 bg-surface p-6">
                <h3 className="mb-4 text-sm font-semibold text-white">LLM Cost by Agent (12 months)</h3>
                <div className="h-64">
                  <ResponsiveContainer width="100%" height="100%">
                    <BarChart data={llmCostByAgent}>
                      <CartesianGrid strokeDasharray="3 3" stroke="rgba(255, 255, 255, 0.06)" />
                      <XAxis
                        dataKey="month"
                        stroke="rgba(255, 255, 255, 0.3)"
                        fontSize={10}
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
                        formatter={(value: number) => [`$${value.toFixed(2)}`, ""]}
                      />
                      <Bar dataKey="forge" stackId="a" fill={agentColors.forge} radius={[0, 0, 0, 0]} />
                      <Bar dataKey="muse" stackId="a" fill={agentColors.muse} />
                      <Bar dataKey="sage" stackId="a" fill={agentColors.sage} />
                      <Bar dataKey="helios" stackId="a" fill={agentColors.helios} radius={[4, 4, 0, 0]} />
                    </BarChart>
                  </ResponsiveContainer>
                </div>
                <div className="mt-3 flex justify-center gap-4 text-xs">
                  {agentPieData.map((item) => (
                    <span key={item.name} className="flex items-center gap-1.5 text-white/60">
                      <span className="h-2 w-2 rounded-full" style={{ backgroundColor: item.color }} />
                      {item.name} (${item.value.toFixed(2)})
                    </span>
                  ))}
                </div>
              </div>
            </div>

            <div className="grid grid-cols-1 gap-6 lg:grid-cols-3">
              <div className="rounded-xl border border-white/10 bg-surface p-6 lg:col-span-2">
                <h3 className="mb-4 text-sm font-semibold text-white">Project Health Over Time</h3>
                <div className="h-64">
                  <ResponsiveContainer width="100%" height="100%">
                    <AreaChart data={projectHealthOverTime}>
                      <defs>
                        <linearGradient id="healthyGrad" x1="0" y1="0" x2="0" y2="1">
                          <stop offset="5%" stopColor="#22c55e" stopOpacity={0.3} />
                          <stop offset="95%" stopColor="#22c55e" stopOpacity={0} />
                        </linearGradient>
                        <linearGradient id="degradedGrad" x1="0" y1="0" x2="0" y2="1">
                          <stop offset="5%" stopColor="#f59e0b" stopOpacity={0.3} />
                          <stop offset="95%" stopColor="#f59e0b" stopOpacity={0} />
                        </linearGradient>
                        <linearGradient id="downGrad" x1="0" y1="0" x2="0" y2="1">
                          <stop offset="5%" stopColor="#ef4444" stopOpacity={0.3} />
                          <stop offset="95%" stopColor="#ef4444" stopOpacity={0} />
                        </linearGradient>
                      </defs>
                      <CartesianGrid strokeDasharray="3 3" stroke="rgba(255, 255, 255, 0.06)" />
                      <XAxis
                        dataKey="month"
                        stroke="rgba(255, 255, 255, 0.3)"
                        fontSize={10}
                        tickLine={false}
                        axisLine={false}
                      />
                      <YAxis
                        stroke="rgba(255, 255, 255, 0.3)"
                        fontSize={11}
                        tickLine={false}
                        axisLine={false}
                      />
                      <Tooltip contentStyle={tooltipStyle} />
                      <Area
                        type="monotone"
                        dataKey="healthy"
                        stackId="1"
                        stroke={healthColors.healthy}
                        fill="url(#healthyGrad)"
                        strokeWidth={2}
                      />
                      <Area
                        type="monotone"
                        dataKey="degraded"
                        stackId="1"
                        stroke={healthColors.degraded}
                        fill="url(#degradedGrad)"
                        strokeWidth={2}
                      />
                      <Area
                        type="monotone"
                        dataKey="down"
                        stackId="1"
                        stroke={healthColors.down}
                        fill="url(#downGrad)"
                        strokeWidth={2}
                      />
                    </AreaChart>
                  </ResponsiveContainer>
                </div>
                <div className="mt-3 flex justify-center gap-4 text-xs">
                  <span className="flex items-center gap-1.5 text-white/60">
                    <span className="h-2 w-2 rounded-full bg-green-500" />
                    Healthy
                  </span>
                  <span className="flex items-center gap-1.5 text-white/60">
                    <span className="h-2 w-2 rounded-full bg-yellow-500" />
                    Degraded
                  </span>
                  <span className="flex items-center gap-1.5 text-white/60">
                    <span className="h-2 w-2 rounded-full bg-red-500" />
                    Down
                  </span>
                </div>
              </div>

              <div className="rounded-xl border border-white/10 bg-surface p-6">
                <h3 className="mb-4 text-sm font-semibold text-white">Provider Split (Mar)</h3>
                <div className="h-52">
                  <ResponsiveContainer width="100%" height="100%">
                    <PieChart>
                      <Pie
                        data={pieData}
                        cx="50%"
                        cy="50%"
                        innerRadius={50}
                        outerRadius={75}
                        paddingAngle={4}
                        dataKey="value"
                      >
                        {pieData.map((entry) => (
                          <Cell key={entry.name} fill={entry.color} />
                        ))}
                      </Pie>
                      <Tooltip
                        contentStyle={tooltipStyle}
                        formatter={(value: number) => [`$${value.toFixed(2)}`, ""]}
                      />
                    </PieChart>
                  </ResponsiveContainer>
                </div>
              </div>
            </div>
          </motion.div>
        </main>
      </div>
    </div>
  );
}
