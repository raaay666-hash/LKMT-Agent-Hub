import { defineConfig } from 'vite';
import { realpathSync } from 'node:fs';
export default defineConfig({root:realpathSync(process.cwd()),server:{port:5173,strictPort:true,watch:{ignored:['**/src-tauri/target/**']}}});
