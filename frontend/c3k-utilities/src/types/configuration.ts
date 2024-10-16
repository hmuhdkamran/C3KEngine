export interface IConfiguration {
    application: string;
    company: string;
    logo: {
        type: string;
        props: React.ImgHTMLAttributes<HTMLImageElement>;
    };
    enableI18n: boolean;
    language: string;
    isRtl: boolean;
    socialMedia: Array<{ name: string, link: string, icon: string }>;
};