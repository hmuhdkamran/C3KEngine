import { type App } from 'vue'
import { createI18n } from 'vue-i18n'

export const installI18n = (app: App, languages: string[] = ['en', 'fa', 'tr', 'ar', 'ch', 'de', 'ur']) => {
    const messages = Object.fromEntries(
        Object.entries(
            import.meta.glob<{ default: any}>('../../locales/*.y(a)?ml', { eager: true }))
            .map(([key, value]) => {
                const yaml = key.endsWith('.yaml')
                return [key.slice(14, yaml ? -5 : -4), value.default]
            }),
    )

    const storedValue = localStorage.getItem('layout')
    let locale = 'en'
    if (storedValue) {
        const parsed = JSON.parse(storedValue)
        if (parsed && Object.prototype.hasOwnProperty.call(parsed, 'activeLanguage')) {
            if (languages.includes(parsed.activeLanguage))
                locale = parsed.activeLanguage
        }
    }
    const i18n = createI18n({
        legacy: false,
        locale,
        messages,
    })

    app.use(i18n)
}