const months = [
  "Apr 2025",
  "May 2025",
  "Jun 2025",
  "Jul 2025",
  "Aug 2025",
  "Sep 2025",
  "Oct 2025",
  "Nov 2025",
  "Dec 2025",
  "Jan 2026",
  "Feb 2026",
  "Mar 2026",
];

export const releaseVelocity = months.map((month, i) => ({
  month,
  releases: [5, 7, 6, 9, 8, 10, 8, 12, 6, 15, 18, 22][i],
}));

export const cycleTimeTrend = months.map((month, i) => ({
  month,
  hours: [72, 68, 62, 58, 52, 48, 48, 42, 38, 32, 28, 24][i],
}));

export const llmCostByProvider = months.map((month, i) => {
  const claude = [45, 52, 48, 62, 58, 72, 68, 85, 78, 110, 128, 142.5][i];
  const gemini = [20, 22, 25, 28, 30, 32, 35, 38, 40, 48, 58, 68.3][i];
  const openai = [15, 18, 16, 20, 22, 24, 26, 28, 22, 30, 32, 34.2][i];
  return {
    month,
    claude,
    gemini,
    openai,
    total: claude + gemini + openai,
  };
});

export const llmCostByAgent = months.map((month, i) => {
  const forge = [35, 42, 38, 52, 48, 60, 55, 70, 62, 88, 102, 118][i];
  const muse = [18, 20, 22, 24, 26, 28, 30, 32, 34, 38, 42, 48][i];
  const sage = [15, 16, 18, 20, 22, 24, 26, 28, 30, 34, 38, 42][i];
  const helios = [12, 14, 12, 14, 14, 16, 18, 21, 14, 28, 36, 37][i];
  return {
    month,
    forge,
    muse,
    sage,
    helios,
    total: forge + muse + sage + helios,
  };
});

export const projectHealthOverTime = months.map((month, i) => {
  const healthy = [18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29][i];
  const degraded = [5, 4, 4, 3, 3, 3, 3, 2, 2, 2, 2, 2][i];
  const down = [2, 2, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1][i];
  return { month, healthy, degraded, down };
});
