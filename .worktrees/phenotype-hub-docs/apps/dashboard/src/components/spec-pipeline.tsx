"use client";

import { motion } from "framer-motion";
import { ArrowRight } from "lucide-react";
import { specPipeline, stages } from "@/data/specs";

export function SpecPipeline() {
  const total = Object.values(specPipeline).reduce((sum, v) => sum + v, 0);
  const maxCount = Math.max(...Object.values(specPipeline));

  return (
    <div className="rounded-xl border border-white/10 bg-surface">
      <div className="flex items-center justify-between border-b border-white/10 px-6 py-4">
        <h2 className="text-lg font-semibold text-white">Spec Pipeline</h2>
        <span className="text-xs text-white/50">
          {total} total specs
        </span>
      </div>

      <div className="p-6">
        <div className="space-y-4">
          {stages.map((stage, i) => {
            const count = specPipeline[stage.key as keyof typeof specPipeline];
            const width = (count / maxCount) * 100;

            return (
              <motion.div
                key={stage.key}
                initial={{ opacity: 0, x: -10 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ delay: i * 0.08, duration: 0.3 }}
                className="group"
              >
                <div className="flex items-center justify-between text-sm">
                  <span className="text-white/70">{stage.label}</span>
                  <span className="font-mono text-white/90">{count}</span>
                </div>
                <div className="mt-1.5 h-8 overflow-hidden rounded-md bg-white/5">
                  <motion.div
                    initial={{ width: 0 }}
                    animate={{ width: `${width}%` }}
                    transition={{ duration: 0.6, delay: i * 0.08 + 0.2 }}
                    className="h-full rounded-md opacity-80 transition-opacity group-hover:opacity-100"
                    style={{ backgroundColor: stage.color }}
                  />
                </div>
              </motion.div>
            );
          })}
        </div>

        <div className="mt-6 flex items-center justify-between border-t border-white/10 pt-4 text-xs text-white/40">
          <span>Flow: Triage → Shipped</span>
          <ArrowRight className="h-4 w-4" />
        </div>
      </div>
    </div>
  );
}
