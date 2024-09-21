import { VNode } from "vue"

export interface I18nLanguage {
    label: string
    i18nLang: string
};

interface ISocialMedia {
    name: string,
    link: string,
    icon: string
};

export interface IConfiguration {
    application: string,
    logo: VNode,
    enableI18n: boolean,
    language: string,
    isRtl: boolean,
    socialMedia: ISocialMedia[]
};