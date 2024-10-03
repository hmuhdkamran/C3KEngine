import type { IConfiguration } from '@/types/configuration';
import logo from '@/assets/react.svg';

export const config: IConfiguration = {
  application: "Ultimate ERP Solution",
  company: "COMBINE CHIPSoft Corporation",
  logo: {
    type: 'img',
    props: {
      src: logo,
      alt: "Logo",
      className: "inline-block w-10",
    }
  },
  enableI18n: true,
  language: "en-US",
  isRtl: false,
  socialMedia: [
    { name: "Facebook", link: "https://www.facebook.com/hmuhdkamran", icon: "icon-[fa--facebook-square]" },
    { name: "Twitter", link: "https://twitter.com/hmuhdkamran", icon: "icon-[fa--twitter]" },
    { name: "Github", link: "https://github.com/hmuhdkamran", icon: "icon-[fa--github]" },
  ],
};
