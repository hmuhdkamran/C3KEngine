import { h, ref } from 'vue'
import { acceptHMRUpdate, defineStore } from 'pinia'

import { type IUser, DefaultUser } from '@/models'
import { LocalStorageHelper, TokenHelper } from '@/helpers'
import type { IConfiguration } from '@/typings'

import logo from '@/assets/images/vue.svg'

export const useSystemStore = defineStore(
  'system',
  () => {
    const user = ref<IUser>(DefaultUser)
    const toggleSidebar = ref(false)
    const application = ref<IConfiguration>(
      LocalStorageHelper.get('configuration') || {
        name: 'Ultimate ERP Solution',
        logo: h('img', { src: logo, class: 'inline-block w-10' }),
        enableI18n: true,
        language: 'en',
        isRtl: false,
        titleColor: '#265689',
        primaryColor: '#265689',
        backgroundColor: '#265689',
        sidebarColor: '#fff',
        fontSize: '16px',
        fontFamily: "'Arial', sans-serif",
        socialMedia: [
          {
            name: 'Facebook',
            link: 'https://www.facebook.com/hmuhdkamran',
            icon: 'icon-[fa--facebook-square]',
          },
          {
            name: 'Twitter',
            link: 'https://twitter.com/hmuhdkamran',
            icon: 'icon-[fa--twitter]',
          },
          {
            name: 'Github',
            link: 'https://github.com/hmuhdkamran',
            icon: 'icon-[fa--github]',
          },
        ],
      },
    )

    function intilizeColors() {
      document.documentElement.style.setProperty('--primary-color', application.value.primaryColor)
      document.documentElement.style.setProperty('--title-color', application.value.titleColor)
      document.documentElement.style.setProperty(
        '--background-color',
        application.value.backgroundColor,
      )
      document.documentElement.style.setProperty('--sidebar-color', application.value.sidebarColor)
      document.documentElement.style.setProperty('--font-size', application.value.fontSize)
      document.documentElement.style.setProperty('--font-family', application.value.fontFamily)
    }

    function updateUser(usr: IUser | string) {
      if (typeof usr === 'string') user.value = TokenHelper.parseUserToken(usr)
      else user.value = usr
    }

    function updateToggleSidebar(changed: boolean) {
      toggleSidebar.value = changed
    }

    function updateApplication(value: Partial<IConfiguration>) {
      application.value = {
        ...application.value,
        ...value,
        socialMedia: value.socialMedia ?? application.value.socialMedia,
      }
      LocalStorageHelper.set('configuration', application.value)
    }

    return {
      user,
      application,
      toggleSidebar,
      intilizeColors,
      updateUser,
      updateToggleSidebar,
      updateApplication,
    }
  },
  { persist: true },
)

if (import.meta.hot) import.meta.hot.accept(acceptHMRUpdate(useSystemStore, import.meta.hot))
