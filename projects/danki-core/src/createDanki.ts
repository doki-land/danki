import {createApp} from 'vue'
import {createRouter, createWebHistory} from 'vue-router'
import {createPinia} from 'pinia'
import {createFluentVue} from 'fluent-vue'
import type {DankiInstance, DankiOptions} from './types'

// 导入组件
import * as components from './components'

/**
 * 创建Danki实例
 * @param options Danki配置选项
 * @returns Danki实例
 */
export function createDanki(options: DankiOptions = {}): DankiInstance {
    // 创建Vue应用实例
    const app = createApp(options.app || {})

    // 创建或使用路由实例
    const router = options.router || createRouter({
        history: createWebHistory(),
        routes: []
    })

    // 创建或使用Pinia实例
    const pinia = options.pinia || createPinia()

    // 注册路由和状态管理
    app.use(router)
    app.use(pinia)

    // 配置国际化
    if (options.i18n) {
        const i18n = createFluentVue(options.i18n)
        app.use(i18n)
    }

    // 应用主题配置
    if (options.theme) {
        const {
            primary,
            secondary,
            accent,
            background,
            textColor
        } = options.theme

        // 设置CSS变量
        if (primary) document.documentElement.style.setProperty('--danki-primary', primary)
        if (secondary) document.documentElement.style.setProperty('--danki-secondary', secondary)
        if (accent) document.documentElement.style.setProperty('--danki-accent', accent)
        if (background) document.documentElement.style.setProperty('--danki-background', background)
        if (textColor) document.documentElement.style.setProperty('--danki-text-color', textColor)
    }

    // 注册所有组件
    Object.entries(components).forEach(([name, component]) => {
        app.component(name, component as any)
    })

    // 返回Danki实例
    return {
        app,
        router,
        pinia,
        version: '0.0.1'
    }
}