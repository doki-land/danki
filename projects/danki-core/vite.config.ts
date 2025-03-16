import {defineConfig} from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
    plugins: [vue()],
    build: {
        lib: {
            entry: 'src/index.ts',
            name: 'danki',
            fileName: 'danki'
        },
        rollupOptions: {
            external: ['vue', 'vue-router', 'pinia'],
            output: {
                globals: {
                    vue: 'Vue',
                    'vue-router': 'VueRouter',
                    pinia: 'Pinia'
                }
            }
        }
    }
})