export const specPipeline = {
  triage: 5,
  specified: 8,
  inProgress: 12,
  validating: 4,
  shipped: 47,
};

export const stages = [
  { key: "triage", label: "Triage", color: "#6b7280" },
  { key: "specified", label: "Specified", color: "#3b82f6" },
  { key: "inProgress", label: "In Progress", color: "#8b5cf6" },
  { key: "validating", label: "Validating", color: "#f59e0b" },
  { key: "shipped", label: "Shipped", color: "#22c55e" },
] as const;

export type SpecStage = (typeof stages)[number]["key"];
