<script setup lang="ts">
import { useToast } from "primevue/usetoast";
import { computed, onMounted } from "vue";
import { RouterView, useRoute } from "vue-router";

import CookieConsent from "./components/CookieConsent.vue";
import FloatingNavbar from "./components/FloatingNavbar.vue";
import { isAuthRoute } from "./router";
import { useAuthStore } from "./stores/auth";
import { setToastInstance } from "./stores/toast";

const route = useRoute();
const shouldShowNavbar = computed(() => isAuthRoute(route.name));

// Initialize toast instance for the store
onMounted(async () => {
  const toast = useToast();
  setToastInstance(toast);

  // Verify token on app load if user is authenticated
  const authStore = useAuthStore();
  if (authStore.isAuthenticated) {
    await authStore.verifyToken();
  }
});
</script>

<template>
  <RouterView style="width: 100vw; height: 100vh;" />
  <FloatingNavbar v-if="shouldShowNavbar" />
  <Toast position="bottom-center" />
  <CookieConsent />
</template>
