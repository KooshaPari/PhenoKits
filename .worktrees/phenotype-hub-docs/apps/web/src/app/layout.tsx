import type { Metadata } from "next"
import { Inter, JetBrains_Mono } from "next/font/google"
import "./globals.css"

const inter = Inter({ subsets: ["latin"], variable: "--font-inter" })
const jetbrains = JetBrains_Mono({ subsets: ["latin"], variable: "--font-mono" })

export const metadata: Metadata = {
  title: "Phenotype — Agent-Native Operating System",
  description: "The Phenotype ecosystem: a one-person conglomerate powered by AI agents. Infrastructure, governance, and delivery — all automated.",
  keywords: ["phenotype", "AI agents", "infrastructure", "Rust", "TypeScript", "spec-driven development"],
  authors: [{ name: "Koosha Pari" }],
  openGraph: {
    title: "Phenotype — Agent-Native Operating System",
    description: "A one-person conglomerate powered by AI agents.",
    type: "website",
  },
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" className="dark">
      <body className={`${inter.variable} ${jetbrains.variable} font-sans antialiased`}>
        {children}
      </body>
    </html>
  )
}
