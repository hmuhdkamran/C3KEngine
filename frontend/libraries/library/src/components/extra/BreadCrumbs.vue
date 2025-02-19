<script setup lang="ts">

interface Props { breadcrumb: string[];};

const props = defineProps<Props>();

</script>

<template>
  <nav class="p-2">
    <ul class="flex items-center">
      <li v-for="item in props.breadcrumb" :key="item" class="flex items-center">
        <div class="flex items-center space-x-2">
          {{ item }}
          <span v-if="item !== props.breadcrumb[breadcrumb.length - 1]" class="mx-2 text-gray-400">/</span>
        </div>
      </li>
    </ul>
  </nav>
</template>
