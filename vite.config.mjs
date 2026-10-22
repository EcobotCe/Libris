import { defineConfig } from 'vite'
import { cpSync, copyFileSync } from 'node:fs'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = dirname(fileURLToPath(import.meta.url))
const output = resolve(root, 'dist')

export default defineConfig({
  root,
  build: {
    outDir: output,
    emptyOutDir: true
  },
  plugins: [{
    name: 'copy-libris-static-assets',
    writeBundle() {
      copyFileSync(resolve(root, 'sw.js'), resolve(output, 'sw.js'))
      copyFileSync(resolve(root, 'manifest.webmanifest'), resolve(output, 'manifest.webmanifest'))
      cpSync(resolve(root, 'icons'), resolve(output, 'icons'), { recursive: true })
    }
  }]
})