<script setup lang="ts">
import { computed, onMounted } from "vue";
import { RouterView, useRoute } from "vue-router";

import FloatingNavbar from "./components/FloatingNavbar.vue";
import { useLanguageSync } from "./composables/useLanguageSync";
import { isAuthRoute } from "./router";

const route = useRoute();
const shouldShowNavbar = computed(() => isAuthRoute(route.name));

// Initialize language synchronization
const { initializeLocale } = useLanguageSync();
onMounted(() => {
  initializeLocale();
});
</script>

<template>
  <RouterView style="width: 100vw; height: 100vh;" />
  <FloatingNavbar v-if="shouldShowNavbar" />
  <Toast position="bottom-center" />
</template>
