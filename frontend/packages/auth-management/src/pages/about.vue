<script setup lang="ts">
import { UsersService } from '@/services'
import type { IUser } from 'c3k-library';
import { onMounted, ref, type Ref } from 'vue';

const repo: UsersService = new UsersService();
const data: Ref<Array<IUser>> = ref([]);

onMounted(() => {
  repo.GetAll().then(res => data.value = res as IUser[]);
});

</script>

<template>
  <div class="about">
    <h1>This is an about page</h1>

    {{ data }}
  </div>
</template>

<route lang="yaml">
  meta:
    layout: application
    action: read
</route>