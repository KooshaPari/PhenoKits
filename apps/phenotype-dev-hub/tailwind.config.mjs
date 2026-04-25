/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        // Phenotype collection accents (from brand playbook)
        sidekick: {
          50: '#f0f9ff',
          100: '#e0f2fe',
          500: '#0ea5e9',
          600: '#0284c7',
          700: '#0369a1'
        },
        eidolon: {
          50: '#fef5e7',
          100: '#fce8bc',
          500: '#f59e0b',
          600: '#d97706',
          700: '#b45309'
        },
        paginary: {
          50: '#f5f3ff',
          100: '#ede9fe',
          500: '#a855f7',
          600: '#9333ea',
          700: '#7e22ce'
        },
        observably: {
          50: '#f0fdf4',
          100: '#dcfce7',
          500: '#22c55e',
          600: '#16a34a',
          700: '#15803d'
        },
        stashly: {
          50: '#fef2f2',
          100: '#fee2e2',
          500: '#ef4444',
          600: '#dc2626',
          700: '#b91c1c'
        },
        // Product line accents
        focalpoint: {
          500: '#3b82f6',
          600: '#2563eb',
          700: '#1d4ed8'
        },
        agileplus: {
          500: '#8b5cf6',
          600: '#7c3aed',
          700: '#6d28d9',
          900: '#4c1d95'
        },
        tracera: {
          500: '#ec4899',
          600: '#db2777',
          700: '#be185d',
          900: '#500724'
        },
        hwledger: {
          500: '#06b6d4',
          600: '#0891b2',
          700: '#0e7490',
          900: '#0c4a6e'
        }
      },
      fontFamily: {
        sans: ['system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace']
      }
    }
  },
  plugins: []
};
