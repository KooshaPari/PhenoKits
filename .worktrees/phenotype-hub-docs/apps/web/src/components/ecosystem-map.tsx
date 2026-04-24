"use client"

import { motion } from "framer-motion"
import { LucideIcon } from "lucide-react"

interface Layer {
  name: string
  description: string
  crates: string
  color: string
  icon: LucideIcon
}

interface EcosystemMapProps {
  layers: Layer[]
}

const colorMap: Record<string, { bg: string; border: string; text: string; glow: string }> = {
  teal: {
    bg: "bg-teal-500/10",
    border: "border-teal-500/30",
    text: "text-teal-400",
    glow: "shadow-teal-500/20",
  },
  blue: {
    bg: "bg-blue-500/10",
    border: "border-blue-500/30",
    text: "text-blue-400",
    glow: "shadow-blue-500/20",
  },
  purple: {
    bg: "bg-purple-500/10",
    border: "border-purple-500/30",
    text: "text-purple-400",
    glow: "shadow-purple-500/20",
  },
  amber: {
    bg: "bg-amber-500/10",
    border: "border-amber-500/30",
    text: "text-amber-400",
    glow: "shadow-amber-500/20",
  },
  rose: {
    bg: "bg-rose-500/10",
    border: "border-rose-500/30",
    text: "text-rose-400",
    glow: "shadow-rose-500/20",
  },
  emerald: {
    bg: "bg-emerald-500/10",
    border: "border-emerald-500/30",
    text: "text-emerald-400",
    glow: "shadow-emerald-500/20",
  },
}

function LayerCard({ layer, index }: { layer: Layer; index: number }) {
  const colors = colorMap[layer.color] || colorMap.teal
  const Icon = layer.icon

  return (
    <motion.div
      initial={{ opacity: 0, x: index % 2 === 0 ? -30 : 30 }}
      whileInView={{ opacity: 1, x: 0 }}
      viewport={{ once: true, margin: "-50px" }}
      transition={{ duration: 0.5, delay: index * 0.1 }}
      whileHover={{ scale: 1.02, y: -4 }}
      className={`relative rounded-xl border ${colors.border} ${colors.bg} p-6 backdrop-blur-sm shadow-lg ${colors.glow} transition-shadow hover:shadow-xl`}
    >
      <div className="flex items-start gap-4">
        <div className={`flex h-12 w-12 shrink-0 items-center justify-center rounded-lg ${colors.bg} ${colors.text}`}>
          <Icon className="h-6 w-6" />
        </div>
        <div className="flex-1">
          <div className="flex items-center justify-between">
            <h3 className="text-lg font-semibold text-white">{layer.name}</h3>
            <span className={`text-sm font-medium ${colors.text}`}>{layer.crates}</span>
          </div>
          <p className="mt-2 text-sm text-white/60">{layer.description}</p>
        </div>
      </div>
    </motion.div>
  )
}

function ConnectorLine({ index }: { index: number }) {
  return (
    <motion.div
      initial={{ opacity: 0, scaleY: 0 }}
      whileInView={{ opacity: 1, scaleY: 1 }}
      viewport={{ once: true }}
      transition={{ duration: 0.4, delay: index * 0.1 + 0.2 }}
      className="flex h-8 items-center justify-center"
    >
      <div className="h-full w-px bg-gradient-to-b from-white/20 to-white/5" />
    </motion.div>
  )
}

export function EcosystemMap({ layers }: EcosystemMapProps) {
  return (
    <div className="relative mx-auto max-w-3xl">
      {layers.map((layer, index) => (
        <motion.div
          key={layer.name}
          initial={{ opacity: 0 }}
          whileInView={{ opacity: 1 }}
          viewport={{ once: true }}
          transition={{ duration: 0.6, delay: index * 0.1 }}
        >
          <LayerCard layer={layer} index={index} />
          {index < layers.length - 1 && <ConnectorLine index={index} />}
        </motion.div>
      ))}
    </div>
  )
}
