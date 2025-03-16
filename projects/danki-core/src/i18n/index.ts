import {createFluentVue} from 'fluent-vue'
import {FluentBundle} from '@fluent/bundle'
import {FluentResource} from "@fluent/bundle/esm/resource";

// 创建语言包实例
const zhHansBundle = new FluentBundle('zh-Hans')
const enUSBundle = new FluentBundle('en-US')

// 加载语言资源
const zhHansResource = await fetch('/src/i18n/locales/zh-Hans.ftl').then(r => r.text())
const enUSResource = await fetch('/src/i18n/locales/en-US.ftl').then(r => r.text())

// 添加资源到语言包
zhHansBundle.addResource(new FluentResource(zhHansResource))
enUSBundle.addResource(new FluentResource(enUSResource))

// 创建fluent-vue实例
const fluentVue = createFluentVue({
    bundles: [zhHansBundle, enUSBundle],
    // @ts-ignore
    locale: 'zh-Hans',
    fallbackLocale: 'en-US'
})

export default fluentVue