"use client";

import { Bell, Search } from "lucide-react";

export function DashboardHeader() {
  return (
    <header className="sticky top-0 z-30 flex h-16 items-center justify-between border-b border-white/10 bg-background/80 px-6 backdrop-blur-md">
      <div className="flex items-center gap-4">
        <div className="relative">
          <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-white/40" />
          <input
            type="text"
            placeholder="Search projects, specs, agents..."
            className="w-80 rounded-lg border border-white/10 bg-surface py-2 pl-10 pr-4 text-sm text-white placeholder:text-white/40 focus:border-primary/50 focus:outline-none focus:ring-1 focus:ring-primary/20"
          />
        </div>
      </div>

      <div className="flex items-center gap-4">
        <button
          type="button"
          className="relative rounded-lg p-2 text-white/60 transition-colors hover:bg-white/5 hover:text-white"
        >
          <Bell className="h-5 w-5" />
          <span className="absolute right-1.5 top-1.5 h-2 w-2 rounded-full bg-primary" />
        </button>

        <div className="flex items-center gap-3 border-l border-white/10 pl-4">
          <div className="text-right">
            <div className="text-sm font-medium text-white">Koosha Pari</div>
            <div className="text-xs text-white/50">Solo Engineer</div>
          </div>
          <div className="flex h-9 w-9 items-center justify-center rounded-full bg-primary/20 text-sm font-semibold text-primary">
            KP
          </div>
        </div>
      </div>
    </header>
  );
}
