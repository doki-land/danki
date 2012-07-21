import type {App} from 'vue'
import type {Router} from 'vue-router'
import type {Pinia} from 'pinia'
// @ts-ignore
import type {FluentVueOptions} from 'fluent-vue'

// Danki类型定义
export interface DankiOptions {
    router?: Router
    pinia?: Pinia
    i18n?: FluentVueOptions
}

export interface DankiInstance {
    app: App
    router: Router
    pinia: Pinia
    mount: (container: string | Element) => void
}

// 基础类型定义
export interface Post {
    id: string
    title: string
    content: string
    author: User
    createdAt: Date
    updatedAt: Date
    comments: Comment[]
    likes: number
}

export interface Comment {
    id: string
    content: string
    author: User
    createdAt: Date
    updatedAt: Date
    likes: number
}

export interface User {
    id: string
    username: string
    avatar: string
    createdAt: Date
}

// Danki配置选项接口
export interface DankiOptions {
    // 路由配置
    router?: Router
    // 状态管理配置
    pinia?: Pinia
    // 国际化配置
    i18n?: FluentVueOptions
    // 主题配置
    theme?: {
        primary?: string
        secondary?: string
        accent?: string
        background?: string
        textColor?: string
    }
}

// Danki实例接口
export interface DankiInstance {
    app: App
    router: Router
    pinia: Pinia
}