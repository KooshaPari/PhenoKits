"use client"

import { useState } from "react"
import Link from "next/link"
import { motion } from "framer-motion"
import {
  ArrowRight,
  Github,
  BookOpen,
  Layers,
  Activity,
  Shield,
  Cpu,
  Terminal,
  Zap,
  Globe,
  Menu,
  X,
} from "lucide-react"
import { Button } from "@phenotype/ui/button"
import { Badge } from "@phenotype/ui/badge"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@phenotype/ui/card"
import { EcosystemMap } from "@/components/ecosystem-map"
import { HeroKeyboard } from "@/components/hero-keyboard"
import { ProjectCard } from "@/components/project-card"
import { projects } from "@/data/projects"

const navItems = [
  { name: "Ecosystem", href: "#ecosystem" },
  { name: "Projects", href: "#projects" },
  { name: "Architecture", href: "#architecture" },
  { name: "Docs", href: "#docs" },
]

const stats = [
  { label: "Projects", value: "30+", icon: Layers },
  { label: "Rust Crates", value: "63", icon: Cpu },
  { label: "TS Packages", value: "3", icon: Globe },
  { label: "AI Agents", value: "4", icon: Zap },
]

const layers = [
  {
    name: "Infrastructure",
    description: "Event sourcing, caching, policy engine, state machines, health checks",
    crates: "17 crates",
    color: "teal",
    icon: Shield,
  },
  {
    name: "Platform",
    description: "System bootstrap, agent orchestration, governance enforcement",
    crates: "28 crates",
    color: "blue",
    icon: Terminal,
  },
  {
    name: "Project Management",
    description: "Spec-driven development, AI agent dispatch, evidence collection",
    crates: "40+ crates",
    color: "purple",
    icon: Activity,
  },
  {
    name: "AI Agents",
    description: "Coding agent CLI, multi-agent orchestration, HTTP API",
    crates: "60+ crates",
    color: "amber",
    icon: Zap,
  },
  {
    name: "LLM Routing",
    description: "Multi-provider OpenAI-compatible proxy",
    crates: "Go service",
    color: "rose",
    icon: Cpu,
  },
  {
    name: "SDKs",
    description: "TypeScript and Python SDKs for all Phenotype primitives",
    crates: "9 packages",
    color: "emerald",
    icon: Layers,
  },
]

export default function Home() {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false)

  return (
    <div className="min-h-screen bg-[#090a0c] text-[#f6f5f5]">
      <header className="sticky top-0 z-50 border-b border-white/10 bg-[#090a0c]/80 backdrop-blur-md">
        <div className="mx-auto flex h-16 max-w-7xl items-center justify-between px-4">
          <Link href="/" className="flex items-center gap-2 font-bold text-xl">
            <span className="text-[#7ebab5]">Phenotype</span>
          </Link>

          <nav className="hidden md:flex items-center gap-8">
            {navItems.map((item) => (
              <a
                key={item.name}
                href={item.href}
                className="text-sm text-white/60 transition-colors hover:text-[#7ebab5]"
              >
                {item.name}
              </a>
            ))}
          </nav>

          <div className="flex items-center gap-3">
            <Button
              variant="outline"
              size="sm"
              asChild
              className="hidden sm:inline-flex border-white/20 text-white hover:bg-white/10"
            >
              <Link href="https://github.com/KooshaPari" target="_blank">
                <Github className="mr-2 h-4 w-4" />
                GitHub
              </Link>
            </Button>
            <Button size="sm" asChild className="bg-[#7ebab5] text-[#090a0c] hover:bg-[#7ebab5]/90">
              <Link href="#ecosystem">
                Explore <ArrowRight className="ml-1 h-4 w-4" />
              </Link>
            </Button>
            <button
              className="md:hidden text-white"
              onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
            >
              {mobileMenuOpen ? <X className="h-6 w-6" /> : <Menu className="h-6 w-4" />}
            </button>
          </div>
        </div>
      </header>

      <main>
        {/* Hero */}
        <section className="relative overflow-hidden">
          <div className="mx-auto max-w-7xl px-4 py-24 md:py-32">
            <div className="grid md:grid-cols-2 gap-12 items-center">
              <div className="space-y-8">
                <motion.div
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ duration: 0.6 }}
                >
                  <Badge className="bg-[#7ebab5]/20 text-[#7ebab5] border-[#7ebab5]/30">
                    Agent-Native Operating System
                  </Badge>
                </motion.div>

                <motion.h1
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ duration: 0.6, delay: 0.1 }}
                  className="text-4xl md:text-6xl font-bold tracking-tight"
                >
                  One person.
                  <br />
                  <span className="text-[#7ebab5]">30+ projects.</span>
                  <br />
                  Zero employees.
                </motion.h1>

                <motion.p
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ duration: 0.6, delay: 0.2 }}
                  className="text-lg text-white/70 max-w-lg"
                >
                  Phenotype is a software engineering organization powered by AI agents.
                  Infrastructure, governance, and delivery — all automated. A one-person conglomerate
                  that scales from startup to enterprise without architectural change.
                </motion.p>

                <motion.div
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ duration: 0.6, delay: 0.3 }}
                  className="flex flex-wrap gap-3"
                >
                  <Button size="lg" asChild className="bg-[#7ebab5] text-[#090a0c] hover:bg-[#7ebab5]/90">
                    <Link href="#ecosystem">
                      Explore Ecosystem <ArrowRight className="ml-2 h-4 w-4" />
                    </Link>
                  </Button>
                  <Button
                    variant="outline"
                    size="lg"
                    asChild
                    className="border-white/20 text-white hover:bg-white/10"
                  >
                    <Link href="#docs">
                      <BookOpen className="mr-2 h-4 w-4" />
                      Documentation
                    </Link>
                  </Button>
                </motion.div>
              </div>

              <motion.div
                initial={{ opacity: 0, scale: 0.9 }}
                animate={{ opacity: 1, scale: 1 }}
                transition={{ duration: 0.8, delay: 0.4 }}
                className="relative"
              >
                <HeroKeyboard />
              </motion.div>
            </div>
          </div>

          <div className="absolute inset-0 bg-gradient-to-b from-transparent via-transparent to-[#090a0c] pointer-events-none" />
        </section>

        {/* Stats */}
        <section className="border-y border-white/10 bg-white/[0.02]">
          <div className="mx-auto max-w-7xl px-4 py-12">
            <div className="grid grid-cols-2 md:grid-cols-4 gap-8">
              {stats.map((stat, i) => (
                <motion.div
                  key={stat.label}
                  initial={{ opacity: 0, y: 20 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  viewport={{ once: true }}
                  transition={{ delay: i * 0.1 }}
                  className="text-center"
                >
                  <stat.icon className="mx-auto h-6 w-6 text-[#7ebab5] mb-2" />
                  <div className="text-3xl font-bold text-[#7ebab5]">{stat.value}</div>
                  <div className="text-sm text-white/50">{stat.label}</div>
                </motion.div>
              ))}
            </div>
          </div>
        </section>

        {/* Ecosystem Map */}
        <section id="ecosystem" className="py-24">
          <div className="mx-auto max-w-7xl px-4">
            <div className="text-center mb-16">
              <Badge className="bg-[#7ebab5]/20 text-[#7ebab5] border-[#7ebab5]/30 mb-4">
                Ecosystem
              </Badge>
              <h2 className="text-3xl md:text-4xl font-bold mt-4">
                The <span className="text-[#7ebab5]">Phenotype Stack</span>
              </h2>
              <p className="text-white/60 mt-4 max-w-2xl mx-auto">
                Six architectural layers, from bare-metal infrastructure to AI agent orchestration.
                Each layer is independently consumable and governed by spec-driven development.
              </p>
            </div>
            <EcosystemMap layers={layers} />
          </div>
        </section>

        {/* Projects */}
        <section id="projects" className="py-24 bg-white/[0.02] border-y border-white/10">
          <div className="mx-auto max-w-7xl px-4">
            <div className="text-center mb-16">
              <Badge className="bg-[#7ebab5]/20 text-[#7ebab5] border-[#7ebab5]/30 mb-4">
                Projects
              </Badge>
              <h2 className="text-3xl md:text-4xl font-bold mt-4">
                Featured <span className="text-[#7ebab5]">Projects</span>
              </h2>
              <p className="text-white/60 mt-4 max-w-2xl mx-auto">
                The core building blocks of the Phenotype ecosystem.
              </p>
            </div>
            <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
              {projects.map((project, i) => (
                <ProjectCard key={project.name} project={project} index={i} />
              ))}
            </div>
          </div>
        </section>

        {/* Architecture */}
        <section id="architecture" className="py-24">
          <div className="mx-auto max-w-7xl px-4">
            <div className="text-center mb-16">
              <Badge className="bg-[#7ebab5]/20 text-[#7ebab5] border-[#7ebab5]/30 mb-4">
                Architecture
              </Badge>
              <h2 className="text-3xl md:text-4xl font-bold mt-4">
                How It <span className="text-[#7ebab5]">Works</span>
              </h2>
            </div>

            <div className="grid md:grid-cols-2 gap-8">
              <Card className="bg-white/[0.03] border-white/10">
                <CardHeader>
                  <CardTitle className="text-[#7ebab5]">Hexagonal Architecture</CardTitle>
                  <CardDescription className="text-white/60">
                    Every crate and package follows ports-and-adapters. Core domain logic is isolated
                    from infrastructure concerns. No inter-crate dependencies within infrakit.
                  </CardDescription>
                </CardHeader>
                <CardContent className="space-y-3 text-sm text-white/70">
                  <div className="flex items-center gap-2">
                    <div className="w-2 h-2 rounded-full bg-[#7ebab5]" />
                    Each crate independently consumable
                  </div>
                  <div className="flex items-center gap-2">
                    <div className="w-2 h-2 rounded-full bg-[#7ebab5]" />
                    Full type annotations, no impl Trait in public APIs
                  </div>
                  <div className="flex items-center gap-2">
                    <div className="w-2 h-2 rounded-full bg-[#7ebab5]" />
                    thiserror with proper #[from] conversions
                  </div>
                </CardContent>
              </Card>

              <Card className="bg-white/[0.03] border-white/10">
                <CardHeader>
                  <CardTitle className="text-[#7ebab5]">Spec-Driven Development</CardTitle>
                  <CardDescription className="text-white/60">
                    AgilePlus treats specifications as executable contracts. Features flow from triage
                    through specification, AI implementation, validation, and shipping.
                  </CardDescription>
                </CardHeader>
                <CardContent className="space-y-3 text-sm text-white/70">
                  <div className="flex items-center gap-2">
                    <div className="w-2 h-2 rounded-full bg-[#7ebab5]" />
                    Evidence trails block state transitions
                  </div>
                  <div className="flex items-center gap-2">
                    <div className="w-2 h-2 rounded-full bg-[#7ebab5]" />
                    AI agents work in isolated worktrees
                  </div>
                  <div className="flex items-center gap-2">
                    <div className="w-2 h-2 rounded-full bg-[#7ebab5]" />
                    Immutable audit trails with SHA-256 hash chains
                  </div>
                </CardContent>
              </Card>

              <Card className="bg-white/[0.03] border-white/10">
                <CardHeader>
                  <CardTitle className="text-[#7ebab5]">Agent-Native Operations</CardTitle>
                  <CardDescription className="text-white/60">
                    Forge, Muse, Sage, and Helios operate as trusted agents with defined roles,
                    coordination protocols, and quality gates.
                  </CardDescription>
                </CardHeader>
                <CardContent className="space-y-3 text-sm text-white/70">
                  <div className="flex items-center gap-2">
                    <div className="w-2 h-2 rounded-full bg-[#7ebab5]" />
                    Single-threaded task ownership
                  </div>
                  <div className="flex items-center gap-2">
                    <div className="w-2 h-2 rounded-full bg-[#7ebab5]" />
                    Cost caps and quality gates per agent
                  </div>
                  <div className="flex items-center gap-2">
                    <div className="w-2 h-2 rounded-full bg-[#7ebab5]" />
                    Multi-provider LLM routing via cliproxyapi++
                  </div>
                </CardContent>
              </Card>

              <Card className="bg-white/[0.03] border-white/10">
                <CardHeader>
                  <CardTitle className="text-[#7ebab5]">Multi-Language SDKs</CardTitle>
                  <CardDescription className="text-white/60">
                    First-class support for Rust, TypeScript, Python, and Go. Each SDK mirrors
                    the same domain model across languages.
                  </CardDescription>
                </CardHeader>
                <CardContent className="space-y-3 text-sm text-white/70">
                  <div className="flex items-center gap-2">
                    <div className="w-2 h-2 rounded-full bg-[#7ebab5]" />
                    @phenotype/pheno-* on npm
                  </div>
                  <div className="flex items-center gap-2">
                    <div className="w-2 h-2 rounded-full bg-[#7ebab5]" />
                    pheno-* on PyPI
                  </div>
                  <div className="flex items-center gap-2">
                    <div className="w-2 h-2 rounded-full bg-[#7ebab5]" />
                    pheno-cli for release governance
                  </div>
                </CardContent>
              </Card>
            </div>
          </div>
        </section>

        {/* Docs */}
        <section id="docs" className="py-24 bg-white/[0.02] border-y border-white/10">
          <div className="mx-auto max-w-7xl px-4 text-center">
            <Badge className="bg-[#7ebab5]/20 text-[#7ebab5] border-[#7ebab5]/30 mb-4">
              Documentation
            </Badge>
            <h2 className="text-3xl md:text-4xl font-bold mt-4">
              Start <span className="text-[#7ebab5]">Building</span>
            </h2>
            <p className="text-white/60 mt-4 max-w-2xl mx-auto mb-12">
              Each project has its own documentation site. Start with the guides most relevant to your needs.
            </p>
            <div className="grid md:grid-cols-3 gap-6 max-w-3xl mx-auto">
              {[
                { name: "AgilePlus", desc: "Spec-driven development engine", href: "#" },
                { name: "thegent", desc: "Platform bootstrap & governance", href: "#" },
                { name: "heliosCLI", desc: "AI coding agent CLI", href: "#" },
              ].map((doc) => (
                <Card key={doc.name} className="bg-white/[0.03] border-white/10 hover:border-[#7ebab5]/30 transition-colors">
                  <CardHeader>
                    <CardTitle className="text-[#7ebab5]">{doc.name}</CardTitle>
                    <CardDescription className="text-white/60">{doc.desc}</CardDescription>
                  </CardHeader>
                  <CardContent>
                    <Button variant="outline" size="sm" asChild className="border-white/20 text-white hover:bg-white/10">
                      <Link href={doc.href}>
                        <BookOpen className="mr-2 h-4 w-4" />
                        Read Docs
                      </Link>
                    </Button>
                  </CardContent>
                </Card>
              ))}
            </div>
          </div>
        </section>

        {/* CTA */}
        <section className="py-24">
          <div className="mx-auto max-w-3xl px-4 text-center">
            <h2 className="text-3xl md:text-4xl font-bold">
              Ready to explore the <span className="text-[#7ebab5]">ecosystem</span>?
            </h2>
            <p className="text-white/60 mt-4 mb-8">
              Dive into the code, read the specs, or just watch the agents work.
            </p>
            <div className="flex flex-wrap justify-center gap-4">
              <Button size="lg" asChild className="bg-[#7ebab5] text-[#090a0c] hover:bg-[#7ebab5]/90">
                <Link href="https://github.com/KooshaPari" target="_blank">
                  <Github className="mr-2 h-4 w-4" />
                  Browse Repos
                </Link>
              </Button>
              <Button
                variant="outline"
                size="lg"
                asChild
                className="border-white/20 text-white hover:bg-white/10"
              >
                <Link href="/dashboard">
                  <Activity className="mr-2 h-4 w-4" />
                  Internal Dashboard
                </Link>
              </Button>
            </div>
          </div>
        </section>
      </main>

      <footer className="border-t border-white/10 py-8">
        <div className="mx-auto max-w-7xl px-4 flex flex-col md:flex-row justify-between items-center gap-4">
          <span className="text-sm text-white/40">
            © 2026 Phenotype. Agent-native since 2023.
          </span>
          <div className="flex items-center gap-4">
            <Link href="https://github.com/KooshaPari" target="_blank" className="text-white/40 hover:text-[#7ebab5] transition-colors">
              <Github className="h-5 w-5" />
            </Link>
          </div>
        </div>
      </footer>
    </div>
  )
}
