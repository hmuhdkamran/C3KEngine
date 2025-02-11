<script setup lang="ts">
import { useSystemStore } from 'c3-library';

const resource = useRoute().path.split('/').pop()
const { t } = useI18n()
const store = useSystemStore();
</script>

<template>
  <Layout class="flex justify-center items-center">
    <div class="error-message">
      <h1>
        4<span class="emoji" />4
      </h1>
      <h2>
        {{ t('common.oops') }} <span v-if="resource" class="gray">'/{{ resource }}'</span> {{ t('common.notFound') }}
      </h2>
      <p>
        {{ t('common.404Message') }}
      </p>

      <router-link to="/dashboard" v-if="store.user.authenticated">
        <n-button round size="large" secondary type="warning" m="3 t8">
          {{ t('menu.dashboard') }}
        </n-button>                
      </router-link>
      <router-link to="/" v-else>
        <n-button round size="large" secondary type="warning" m="3 t8">
          {{ t('common.goHome') }}
        </n-button>
      </router-link>
      <router-link to="/account/login" v-if="!store.user.authenticated">
        <n-button round size="large" secondary type="info" m="3 t8">
          {{ t('login.title') }}
        </n-button>        
      </router-link>
    </div>
  </Layout>
</template>

<route lang="yaml">
meta:
  layout: 404
</route>
