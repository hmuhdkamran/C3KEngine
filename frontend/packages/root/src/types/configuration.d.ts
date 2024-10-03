export interface IConfiguration {
    application: string;
    company: string;
    logo: ReactNode;
    enableI18n: boolean;
    language: string;
    isRtl: boolean;
    socialMedia: Array<{ name: string, link: string, icon: string }>;
};