import {createDanki} from '@doki-land/danki'
import App from './App.vue'

// 创建Danki实例
const danki = createDanki({
    app: App,
    // 配置国际化
    i18n: {
        locales: ['zh-CN', 'en-US'],
        defaultLocale: 'zh-CN',
        // 加载翻译资源
        bundleOptions: {
            functions: {
                // 可以添加自定义函数
            },
        },
        // 可以配置加载翻译文件的方式
        // 这里使用简单的示例
        bundles: {
            'zh-CN': {
                'app': 'welcome = 欢迎使用Danki论坛系统',
            },
            'en-US': {
                'app': 'welcome = Welcome to Danki Forum System',
            },
        },
    },
    // 配置主题
    theme: {
        primary: '#3498db',
        secondary: '#2ecc71',
        accent: '#e74c3c',
        background: '#f8f9fa',
        textColor: '#333333'
    }
})

// 挂载应用
danki.app.mount('#app')