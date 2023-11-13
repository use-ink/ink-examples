import path from 'node:path';
import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';
import dts from 'vite-plugin-dts';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    react(),
    dts({
      insertTypesEntry: true,
    }),
  ],
  build: {
    sourcemap: true,
    lib: {
      entry: path.resolve(__dirname, 'src/index.ts'),
      name: 'ui',
      formats: ['es', 'umd'],
      fileName: (format) => `ui.${format}.js`,
    },
    rollupOptions: {
      external: [
        'react',
        'react-dom',
        'useink',
        'useink/core',
        'useink/notifications',
        'useink/utils',
      ],
      output: {
        globals: {
          react: 'React',
          'react-dom': 'ReactDOM',
          useink: 'useink',
          'useink/core': 'useink/core',
          'useink/notifications': 'useink/notifications',
          'useink/utils': 'useink/utils',
        },
      },
    },
  },
});
