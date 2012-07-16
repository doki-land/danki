import type { App } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'
import { createPinia } from 'pinia'

// 类型定义
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

// 创建路由实例
const router = createRouter({
  history: createWebHistory(),
  routes: []
})

// 创建状态管理实例
const pinia = createPinia()

// 插件安装函数
export function install(app: App) {
  app.use(router)
  app.use(pinia)
}

// 导出版本号
export const version = '0.0.1'