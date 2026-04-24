"use client";

import { motion } from "framer-motion";
import { GitBranch, GitCommit, CheckCircle2, XCircle, AlertCircle, Clock } from "lucide-react";
import { projects, type Project } from "@/data/projects";

const statusColors: Record<string, string> = {
  healthy: "bg-green-500",
  degraded: "bg-yellow-500",
  down: "bg-red-500",
};

const ciColors: Record<string, string> = {
  passing: "text-green-400",
  failing: "text-red-400",
  pending: "text-yellow-400",
};

const ciIcons: Record<string, typeof CheckCircle2> = {
  passing: CheckCircle2,
  failing: XCircle,
  pending: AlertCircle,
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

function ProjectCard({ project, index }: { project: Project; index: number }) {
  const CIIcon = ciIcons[project.ciStatus];

  return (
    <motion.div
      initial={{ opacity: 0, y: 10 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ delay: index * 0.02, duration: 0.3 }}
      whileHover={{ y: -2, borderColor: "rgba(126, 187, 181, 0.3)" }}
      className="rounded-lg border border-white/10 bg-surface p-4 transition-colors"
    >
      <div className="flex items-start justify-between">
        <div className="flex items-center gap-2">
          <div className={`h-2.5 w-2.5 rounded-full ${statusColors[project.status]}`} />
          <span className="text-sm font-medium text-white">{project.name}</span>
        </div>
        <span
          className={`inline-flex items-center gap-1 rounded-md bg-white/5 px-2 py-0.5 text-xs font-mono ${ciColors[project.ciStatus]}`}
        >
          <CIIcon className="h-3 w-3" />
          {project.ciStatus}
        </span>
      </div>

      <div className="mt-3 space-y-1.5 text-xs text-white/50">
        <div className="flex items-center gap-1.5">
          <GitCommit className="h-3.5 w-3.5" />
          <span>{formatRelativeTime(project.lastCommit)}</span>
        </div>
        <div className="flex items-center gap-1.5">
          <GitBranch className="h-3.5 w-3.5" />
          <span className="font-mono">{project.branch}</span>
        </div>
      </div>
    </motion.div>
  );
}

export function ProjectHealth() {
  const healthy = projects.filter((p) => p.status === "healthy").length;
  const degraded = projects.filter((p) => p.status === "degraded").length;
  const down = projects.filter((p) => p.status === "down").length;

  return (
    <div className="rounded-xl border border-white/10 bg-surface">
      <div className="flex items-center justify-between border-b border-white/10 px-6 py-4">
        <h2 className="text-lg font-semibold text-white">Project Health</h2>
        <div className="flex items-center gap-4 text-xs">
          <span className="flex items-center gap-1.5 text-green-400">
            <span className="h-2 w-2 rounded-full bg-green-500" />
            {healthy} healthy
          </span>
          <span className="flex items-center gap-1.5 text-yellow-400">
            <span className="h-2 w-2 rounded-full bg-yellow-500" />
            {degraded} degraded
          </span>
          <span className="flex items-center gap-1.5 text-red-400">
            <span className="h-2 w-2 rounded-full bg-red-500" />
            {down} down
          </span>
        </div>
      </div>

      <div className="grid grid-cols-1 gap-3 p-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
        {projects.map((project, i) => (
          <ProjectCard key={project.name} project={project} index={i} />
        ))}
      </div>
    </div>
  );
}
