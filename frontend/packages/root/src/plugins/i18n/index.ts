import { I18nLanguage } from '@/plugins/typings'
import { createI18n } from 'vue-i18n'

const messages = Object.fromEntries(
  Object.entries(
    import.meta.glob<{ default: any }>('./locales/*.json', { eager: true }))
    .map(([key, value]) => [key.slice(10, -5), value.default]),
)

export const i18nCompLanguages: I18nLanguage[] = [
  {
    label: 'English',
    i18nLang: 'en-US',
  },
  {
    label: 'Urdu',
    i18nLang: 'ur-PK',
  },
]

export default createI18n({
  legacy: false,
  locale: 'en-US',
  fallbackLocale: 'en-US',
  messages,
})
