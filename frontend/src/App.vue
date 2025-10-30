<script setup lang="ts">
import { computed } from "vue";
import { RouterView, useRoute } from "vue-router";

import CookieConsent from "./components/CookieConsent.vue";
import FloatingNavbar from "./components/FloatingNavbar.vue";
import { isAuthRoute } from "./router";

const route = useRoute();
const shouldShowNavbar = computed(() => isAuthRoute(route.name));
</script>

<template>
  <RouterView style="width: 100vw; height: 100vh;" />
  <FloatingNavbar v-if="shouldShowNavbar" />
  <Toast position="bottom-center" />
  <CookieConsent />
</template>
