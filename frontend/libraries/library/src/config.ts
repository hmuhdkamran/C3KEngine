import { h } from "vue";
import type { IConfiguration } from "./typings";
import logo from "@/assets/images/vue.svg"

export const config: IConfiguration = {
    application: "Ultimate ERP Solution",
    logo: h('img', { src: logo, class: "inline-block w-10" }),
    enableI18n: true,
    language: "en-US",
    isRtl: false,
    socialMedia: [
        { name: "Facebook", link: "https://www.facebook.com/hmuhdkamran", icon: "icon-[fa--facebook-square]" },
        { name: "Twitter", link: "https://twitter.com/hmuhdkamran", icon: "icon-[fa--twitter]" },
        { name: "Github", link: "https://github.com/hmuhdkamran", icon: "icon-[fa--github]" },
    ],
}
