import { defineStore } from "pinia";
import { useToast } from "primevue/usetoast";

import i18n from "@/i18n";

export enum ToastSeverity {
  SUCCESS = "success",
  INFO = "info",
  WARN = "warn",
  ERROR = "error",
}

export const useToastStore = defineStore("toast", () => {
  // Lazy getter for toast instance to avoid calling useToast at store definition
  function getToast() {
    return useToast();
  }

  function showError(message: string, life: number = 5000) {
    getToast().add({
      severity: ToastSeverity.ERROR,
      life,
      detail: message,
      summary: i18n.global.t("toast.error"),
    });
  }

  function showInfo(message: string, life: number = 5000) {
    getToast().add({
      severity: ToastSeverity.INFO,
      life,
      detail: message,
      summary: i18n.global.t("toast.info"),
    });
  }

  function showSuccess(message: string, life: number = 5000) {
    getToast().add({
      severity: ToastSeverity.SUCCESS,
      life,
      detail: message,
      summary: i18n.global.t("toast.success"),
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
        return i18n.global.t("toast.error");
      case ToastSeverity.SUCCESS:
        return i18n.global.t("toast.success");
      case ToastSeverity.WARN:
        return i18n.global.t("toast.warn");
      default:
        return i18n.global.t("toast.info");
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
    const message = i18n.global.t(`translation_keys.${translationKey}`);

    getToast().add({
      severity,
      life,
      detail: message,
      summary: getSummaryForSeverity(severity),
    });
  }

  return { showError, showInfo, showSuccess, showTranslationKey };
});
