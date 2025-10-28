import { defineStore } from "pinia";
import { ref } from "vue";
import { useI18n } from "vue-i18n";

import { useToastStore } from "./toast";

import type { Settings, UpdateSettings } from "@/services/settings/types";

import { settingsService } from "@/services/settings";
import { ServerStatus } from "@/services/types";


export const useSettingsStore = defineStore("settings", () => {
  const settings = ref<Settings | null>(null);
  const loading = ref(false);
  const { t } = useI18n();

  async function loadSettings(): Promise<void> {
    loading.value = true;
    try {
      const response = await settingsService.getSettings();

      if (response.status !== ServerStatus.NO_ERROR) {
        useToastStore().showError(t("settings.errors.loadFailed"));
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
        useToastStore().showError(t("settings.errors.updateFailed"));
        return false;
      }

      settings.value = response.data;
      useToastStore().showInfo(t("settings.success.updated"));
      return true;
    } finally {
      loading.value = false;
    }
  }

  return { settings, loading, loadSettings, updateSettings };
});
