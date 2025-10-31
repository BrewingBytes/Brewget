import { defineStore } from "pinia";
import { useToast } from "primevue";
import { useI18n } from "vue-i18n";

import type { SupportedLocale } from "@/i18n";

import { translateKey } from "@/utils/i18n";


export enum ToastSeverity {
  SUCCESS = "success",
  INFO = "info",
  WARN = "warn",
  ERROR = "error",
}

export const useToastStore = defineStore("toast", () => {
  const toast = useToast();
  const { locale, messages } = useI18n();

  function showError(message: string, life: number = 5000) {
    toast.add({
      severity: ToastSeverity.ERROR,
      life,
      detail: message,
      summary: "Error",
    });
  }

  function showInfo(message: string, life: number = 5000) {
    toast.add({
      severity: ToastSeverity.INFO,
      life,
      detail: message,
      summary: "Info",
    });
  }

  function showSuccess(message: string, life: number = 5000) {
    toast.add({
      severity: ToastSeverity.SUCCESS,
      life,
      detail: message,
      summary: "Success",
    });
  }

  /**
   * Gets the appropriate summary for a toast based on severity
   * @param severity The toast severity
   * @returns The summary string
   */
  function getSummaryForSeverity(severity: ToastSeverity): string {
    switch (severity) {
      case ToastSeverity.ERROR:
        return "Error";
      case ToastSeverity.SUCCESS:
        return "Success";
      default:
        return "Info";
    }
  }

  /**
   * Shows a toast with a message from a translation key
   * @param translationKey The backend translation key
   * @param severity The toast severity (defaults to INFO)
   * @param life Toast display duration in milliseconds
   */
  function showTranslationKey(
    translationKey: string,
    severity: ToastSeverity = ToastSeverity.INFO,
    life: number = 5000,
  ) {
    const message = translateKey(
      translationKey,
      locale.value as SupportedLocale,
      messages.value,
    );

    toast.add({
      severity,
      life,
      detail: message,
      summary: getSummaryForSeverity(severity),
    });
  }

  return { showError, showInfo, showSuccess, showTranslationKey };
});
