/**
 * Color class map for Phenotype collections and products.
 * Uses literal class strings so Tailwind's purge can find them.
 * All accent colors from tailwind.config.mjs are mapped to their
 * corresponding Tailwind utility classes.
 */

export type AccentColor =
  | 'sidekick'
  | 'eidolon'
  | 'paginary'
  | 'observably'
  | 'stashly'
  | 'focalpoint'
  | 'agileplus'
  | 'tracera'
  | 'hwledger';

interface ColorClasses {
  bgDark: string;        // bg-{accent}-900/30
  bgAccent: string;      // bg-{accent}-500/20
}

const colorMap: Record<AccentColor, ColorClasses> = {
  sidekick: {
    bgDark: 'bg-sidekick-900/30',
    bgAccent: 'bg-sidekick-500/20'
  },
  eidolon: {
    bgDark: 'bg-eidolon-900/30',
    bgAccent: 'bg-eidolon-500/20'
  },
  paginary: {
    bgDark: 'bg-paginary-900/30',
    bgAccent: 'bg-paginary-500/20'
  },
  observably: {
    bgDark: 'bg-observably-900/30',
    bgAccent: 'bg-observably-500/20'
  },
  stashly: {
    bgDark: 'bg-stashly-900/30',
    bgAccent: 'bg-stashly-500/20'
  },
  focalpoint: {
    bgDark: 'bg-focalpoint-900/30',
    bgAccent: 'bg-focalpoint-500/20'
  },
  agileplus: {
    bgDark: 'bg-agileplus-900/30',
    bgAccent: 'bg-agileplus-500/20'
  },
  tracera: {
    bgDark: 'bg-tracera-900/30',
    bgAccent: 'bg-tracera-500/20'
  },
  hwledger: {
    bgDark: 'bg-hwledger-900/30',
    bgAccent: 'bg-hwledger-500/20'
  }
};

export function getColorClasses(accent: string): ColorClasses {
  const normalized = accent.toLowerCase() as AccentColor;
  return colorMap[normalized] || colorMap.sidekick;
}
