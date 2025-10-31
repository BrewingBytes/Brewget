import { defineStore } from "pinia";
import { ref } from "vue";

import { useToastStore } from "./toast";

import type { Settings, UpdateSettings } from "@/services/settings/types";

import i18n from "@/i18n";
import { settingsService } from "@/services/settings";
import { ServerStatus } from "@/services/types";

export const useSettingsStore = defineStore("settings", () => {
  const settings = ref<Settings | null>(null);
  const loading = ref(false);

  async function loadSettings(): Promise<void> {
    loading.value = true;
    try {
      const response = await settingsService.getSettings();

      if (response.status !== ServerStatus.NO_ERROR) {
        useToastStore().showError(i18n.global.t("settings.failed_to_load"));
        return;
      }

      settings.value = response.data;
    } finally {
      loading.value = false;
    }
  }

  async function updateSettings(updates: UpdateSettings): Promise<boolean> {
    loading.value = true;
    try {
      const response = await settingsService.updateSettings(updates);

      if (response.status !== ServerStatus.NO_ERROR) {
        useToastStore().showError(i18n.global.t("settings.failed_to_update"));
        return false;
      }

      settings.value = response.data;
      useToastStore().showInfo(i18n.global.t("settings.settings_updated"));
      return true;
    } finally {
      loading.value = false;
    }
  }

  return { settings, loading, loadSettings, updateSettings };
});
