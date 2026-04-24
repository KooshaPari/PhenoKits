"use client";

import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  PieChart,
  Pie,
  Cell,
  LineChart,
  Line,
} from "recharts";

const releaseData = [
  { month: "Oct", releases: 8 },
  { month: "Nov", releases: 12 },
  { month: "Dec", releases: 6 },
  { month: "Jan", releases: 15 },
  { month: "Feb", releases: 18 },
  { month: "Mar", releases: 22 },
];

const llmCostData = [
  { name: "Claude", value: 142.5, color: "#d97706" },
  { name: "Gemini", value: 68.3, color: "#3b82f6" },
  { name: "OpenAI", value: 34.2, color: "#22c55e" },
];

const cycleTimeData = [
  { month: "Oct", hours: 48 },
  { month: "Nov", hours: 42 },
  { month: "Dec", hours: 38 },
  { month: "Jan", hours: 32 },
  { month: "Feb", hours: 28 },
  { month: "Mar", hours: 24 },
];

const tooltipStyle = {
  backgroundColor: "#181b21",
  border: "1px solid rgba(255, 255, 255, 0.1)",
  borderRadius: "8px",
  color: "#f6f5f5",
  fontSize: "12px",
};

export function ReleaseVelocityChart() {
  return (
    <div className="h-52">
      <ResponsiveContainer width="100%" height="100%">
        <BarChart data={releaseData}>
          <CartesianGrid strokeDasharray="3 3" stroke="rgba(255, 255, 255, 0.06)" />
          <XAxis
            dataKey="month"
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
          />
          <Tooltip contentStyle={tooltipStyle} />
          <Bar dataKey="releases" fill="#7ebab5" radius={[4, 4, 0, 0]} />
        </BarChart>
      </ResponsiveContainer>
    </div>
  );
}

export function LLmCostChart() {
  return (
    <div className="h-52">
      <ResponsiveContainer width="100%" height="100%">
        <PieChart>
          <Pie
            data={llmCostData}
            cx="50%"
            cy="50%"
            innerRadius={50}
            outerRadius={75}
            paddingAngle={4}
            dataKey="value"
          >
            {llmCostData.map((entry) => (
              <Cell key={`cell-${entry.name}`} fill={entry.color} />
            ))}
          </Pie>
          <Tooltip contentStyle={tooltipStyle} formatter={(value: number) => `$${value.toFixed(2)}`} />
        </PieChart>
      </ResponsiveContainer>
      <div className="mt-2 flex justify-center gap-4 text-xs">
        {llmCostData.map((item) => (
          <span key={item.name} className="flex items-center gap-1.5 text-white/60">
            <span className="h-2 w-2 rounded-full" style={{ backgroundColor: item.color }} />
            {item.name} (${item.value.toFixed(2)})
          </span>
        ))}
      </div>
    </div>
  );
}

export function CycleTimeChart() {
  return (
    <div className="h-52">
      <ResponsiveContainer width="100%" height="100%">
        <LineChart data={cycleTimeData}>
          <CartesianGrid strokeDasharray="3 3" stroke="rgba(255, 255, 255, 0.06)" />
          <XAxis
            dataKey="month"
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
            unit="h"
          />
          <Tooltip contentStyle={tooltipStyle} formatter={(value: number) => `${value}h`} />
          <Line
            type="monotone"
            dataKey="hours"
            stroke="#7ebab5"
            strokeWidth={2}
            dot={{ fill: "#7ebab5", strokeWidth: 2 }}
            activeDot={{ r: 5 }}
          />
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
}
