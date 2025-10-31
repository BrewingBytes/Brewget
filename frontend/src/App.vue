<script setup lang="ts">
import { useToast } from "primevue/usetoast";
import { computed, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { RouterView, useRoute } from "vue-router";

import CookieConsent from "./components/CookieConsent.vue";
import FloatingNavbar from "./components/FloatingNavbar.vue";
import { SUPPORTED_LOCALES } from "./i18n";
import { isAuthRoute } from "./router";
import { useAuthStore } from "./stores/auth";
import { useSettingsStore } from "./stores/settings";
import { setToastInstance } from "./stores/toast";

import type { SupportedLocale } from "./i18n";

const route = useRoute();
const shouldShowNavbar = computed(() => isAuthRoute(route.name));
const { locale } = useI18n();
const authStore = useAuthStore();
const settingsStore = useSettingsStore();

// Initialize toast instance for the store
onMounted(async () => {
  const toast = useToast();
  setToastInstance(toast);

  // Load user settings if authenticated to apply language preference
  if (authStore.isAuthenticated) {
    await settingsStore.loadSettings();
    
    // Apply language preference from settings
    if (settingsStore.settings && SUPPORTED_LOCALES.includes(settingsStore.settings.language as SupportedLocale)) {
      locale.value = settingsStore.settings.language;
    }
  }
});
</script>

<template>
  <RouterView style="width: 100vw; height: 100vh;" />
  <FloatingNavbar v-if="shouldShowNavbar" />
  <Toast position="bottom-center" />
  <CookieConsent />
</template>
