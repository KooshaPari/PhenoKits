"use client"

import Link from "next/link"
import { motion } from "framer-motion"
import { Github } from "lucide-react"
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from "@phenotype/ui/card"
import { Badge } from "@phenotype/ui/badge"
import { Button } from "@phenotype/ui/button"

export interface Project {
  name: string
  description: string
  language: "Rust" | "TypeScript" | "Python" | "Go"
  count: string
  githubUrl: string
  category:
    | "infrastructure"
    | "platform"
    | "management"
    | "agents"
    | "routing"
    | "sdk"
    | "workflow"
    | "devops"
    | "frontend"
}

interface ProjectCardProps {
  project: Project
  index: number
}

const languageColors: Record<string, string> = {
  Rust: "bg-orange-500/20 text-orange-400 border-orange-500/30",
  TypeScript: "bg-blue-500/20 text-blue-400 border-blue-500/30",
  Python: "bg-yellow-500/20 text-yellow-400 border-yellow-500/30",
  Go: "bg-cyan-500/20 text-cyan-400 border-cyan-500/30",
}

export function ProjectCard({ project, index }: ProjectCardProps) {
  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      whileInView={{ opacity: 1, y: 0 }}
      viewport={{ once: true, margin: "-50px" }}
      transition={{ duration: 0.4, delay: index * 0.05 }}
      whileHover={{ y: -8 }}
    >
      <Card className="h-full bg-white/[0.03] border-white/10 hover:border-[#7ebab5]/30 transition-colors">
        <CardHeader>
          <div className="flex items-start justify-between">
            <CardTitle className="text-lg text-white">{project.name}</CardTitle>
            <Badge className={languageColors[project.language]}>
              {project.language}
            </Badge>
          </div>
          <CardDescription className="text-white/60">
            {project.description}
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex items-center gap-2 text-sm text-white/50">
            <span>{project.count}</span>
            <span>•</span>
            <span className="capitalize">{project.category}</span>
          </div>
        </CardContent>
        <CardFooter>
          <Button
            variant="outline"
            size="sm"
            asChild
            className="w-full border-white/20 text-white hover:bg-white/10"
          >
            <Link href={project.githubUrl} target="_blank">
              <Github className="mr-2 h-4 w-4" />
              View on GitHub
            </Link>
          </Button>
        </CardFooter>
      </Card>
    </motion.div>
  )
}
