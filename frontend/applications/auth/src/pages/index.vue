<script setup lang="ts">
import { FeatureSection, GetStartedSection, ModuleSection, SliderSection, SupportSection, TestimonialSection } from "~/components/landing-page";
</script>

<route lang="yaml">
meta:
  name: dashboard
  title: dashboard
  authRequired: false
  breadcrumb:
    - dashboard
</route>

<template>
  <div>
    <SliderSection />
    <GetStartedSection />
    <ModuleSection />
    <SupportSection />
    <FeatureSection />
    <TestimonialSection />
  </div>
</template>

<style scoped></style>
