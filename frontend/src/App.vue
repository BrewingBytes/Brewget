<script setup lang="ts">
import { computed } from "vue";
import { RouterView, useRoute } from "vue-router";

import FloatingNavbar from "./components/FloatingNavbar.vue";
import { useLanguage } from "./composables/useLanguage";
import { isAuthRoute } from "./router";

const route = useRoute();
const shouldShowNavbar = computed(() => isAuthRoute(route.name));

// Initialize language management
useLanguage();
</script>

<template>
  <RouterView style="width: 100vw; height: 100vh;" />
  <FloatingNavbar v-if="shouldShowNavbar" />
  <Toast position="bottom-center" />
</template>
