import {defineConfig} from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
    plugins: [vue()],
    build: {
        sourcemap: true,
    build: {
        lib: {
            entry: 'src/index.ts',
            name: 'danki',
            fileName: 'danki',
            formats: ['es']
        },
        rollupOptions: {
            external: ['vue', 'vue-router', 'pinia', '@fluent/bundle', 'fluent-vue', '@vueuse/core'],
            output: {
                globals: {
                    vue: 'Vue',
                    'vue-router': 'VueRouter',
                    pinia: 'Pinia',
                    '@fluent/bundle': 'FluentBundle',
                    'fluent-vue': 'FluentVue',
                    '@vueuse/core': 'VueUse'
                }
            }
        }
    }
})