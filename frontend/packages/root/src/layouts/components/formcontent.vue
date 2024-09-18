<script setup lang="ts">
import { defineProps, computed } from 'vue';
import { hrmsform } from '@/assets/hrmsformdata'; 
import FormField from './formfeild.vue';

type ModuleTitle = keyof typeof hrmsform;

interface Props {
  moduleTitle: ModuleTitle;
}

const props = defineProps<Props>();

const formContent = computed(() => hrmsform[props.moduleTitle] || {
  title: 'Unknown Module',
  fields: [],
});
</script>

<template>
  <div>
    <h2 class="text-xl font-bold mb-4">{{ formContent.title }} Form</h2>
    <form>
      <FormField v-for="field in formContent.fields" :key="field.label" :field="field" />
      <button type="submit" class="btn-gradient rounded-lg">Submit</button>
    </form>
  </div>
</template>
