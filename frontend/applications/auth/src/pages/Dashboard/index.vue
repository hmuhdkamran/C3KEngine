<script setup lang="ts">
</script>

<route lang="yaml">
meta:
  name: dashboard
  title: dashboard
  layout: auth
  authRequired: true
  breadcrumb:
    - dashboard
</route>

<template>
  <div>
    <DashboardPage />
  </div>
</template>

<style scoped></style>
