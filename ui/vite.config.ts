import { defineConfig } from 'vite';
import path from 'node:path';
import tailwindcss from '@tailwindcss/vite';
import vue from '@vitejs/plugin-vue';

const gatewayOrigin = process.env.GATEWAY_ORIGIN ?? 'http://127.0.0.1:42617';
const gatewayWsOrigin = gatewayOrigin.replace(/^http/, 'ws');

export default defineConfig({
  base: '/_ui/',
  plugins: [vue(), tailwindcss()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  server: {
    strictPort: true,
    proxy: {
      '/health': {
        target: gatewayOrigin,
        changeOrigin: true,
      },
      '/pair': {
        target: gatewayOrigin,
        changeOrigin: true,
      },
      '/api': {
        target: gatewayOrigin,
        changeOrigin: true,
      },
      '/ws': {
        target: gatewayWsOrigin,
        ws: true,
      },
    },
  },
  build: {
    outDir: 'dist',
  },
});
