import { defineStore } from "pinia";
import { ref } from "vue";

import { useToastStore } from "./toast";

import type { Settings, UpdateSettings } from "@/services/settings/types";

import { settingsService } from "@/services/settings";
import { ServerStatus } from "@/services/types";
import { useAuthStore } from "@/stores/auth";
import { getUserIdFromToken } from "@/utils/jwt";


export const useSettingsStore = defineStore("settings", () => {
  const settings = ref<Settings | null>(null);
  const loading = ref(false);

  async function loadSettings(): Promise<void> {
    const authStore = useAuthStore();
    const userId = getUserIdFromToken(authStore.token);

    if (!userId) {
      useToastStore().showError("Failed to get user ID from token.");
      return;
    }

    loading.value = true;
    try {
      const response = await settingsService.getSettings(userId);

      if (response.status !== ServerStatus.NO_ERROR) {
        useToastStore().showError("Failed to load settings.");
        return;
      }

      settings.value = response.data;
    } finally {
      loading.value = false;
    }
  }

  async function updateSettings(updates: UpdateSettings): Promise<boolean> {
    const authStore = useAuthStore();
    const userId = getUserIdFromToken(authStore.token);

    if (!userId) {
      useToastStore().showError("Failed to get user ID from token.");
      return false;
    }

    loading.value = true;
    try {
      const response = await settingsService.updateSettings(userId, updates);

      if (response.status !== ServerStatus.NO_ERROR) {
        useToastStore().showError("Failed to update settings.");
        return false;
      }

      settings.value = response.data;
      useToastStore().showInfo("Settings updated successfully.");
      return true;
    } finally {
      loading.value = false;
    }
  }

  return { settings, loading, loadSettings, updateSettings };
});
