<script setup lang="ts">
import { RouterView } from 'vue-router'
import { Notification, useNotification, useSystemStore } from 'c3-library';
import { computed } from 'vue';

const { notifications } = useNotification();

const notificationsWithOffsets = computed(() =>
  notifications.value.map((notification: any, index: number) => ({
    ...notification,
    positionOffset: index * 110
  }))
);

const store = useSystemStore()
store.intilizeColors();

</script>

<template>
  <div class="font-inter tracking-tight bg-gray-100 text-gray-900 antialiased">
    <Notification v-for="notification in notificationsWithOffsets" :key="notification.id" :title="notification.title"
      :message="notification.message" :type="notification.type" :position="notification.position"
      :positionOffset="notification.positionOffset" :duration="3000" />
    <router-view />
  </div>
</template>

<style scoped>
header {
  line-height: 1.5;
  max-height: 100vh;
}

.logo {
  display: block;
  margin: 0 auto 2rem;
}

nav {
  width: 100%;
  font-size: 12px;
  text-align: center;
  margin-top: 2rem;
}

nav a.router-link-exact-active {
  color: var(--color-text);
}

nav a.router-link-exact-active:hover {
  background-color: transparent;
}

nav a {
  display: inline-block;
  padding: 0 1rem;
  border-left: 1px solid var(--color-border);
}

nav a:first-of-type {
  border: 0;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  .logo {
    margin: 0 2rem 0 0;
  }

  header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }

  nav {
    text-align: left;
    margin-left: -1rem;
    font-size: 1rem;

    padding: 1rem 0;
    margin-top: 1rem;
  }
}
</style>
