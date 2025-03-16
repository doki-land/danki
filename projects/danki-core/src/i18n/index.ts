import {createFluentVue} from 'fluent-vue'
import {FluentBundle} from '@fluent/bundle'
import {FluentResource} from "@fluent/bundle/esm/resource"
import zhHansResource from './locales/zh-Hans.ftl?raw'
import enUSResource from './locales/en-US.ftl?raw'

// 创建语言包实例
const zhHansBundle = new FluentBundle('zh-Hans')
const enUSBundle = new FluentBundle('en-US')

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