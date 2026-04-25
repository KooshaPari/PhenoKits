import { defineConfig } from 'astro/config';
import tailwind from '@astrojs/tailwind';

export default defineConfig({
  integrations: [tailwind()],
  output: 'static',
  site: 'https://projects.kooshapari.com',
  vite: {
    ssr: {
      external: []
    }
  }
});
