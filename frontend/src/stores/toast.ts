import { defineStore } from "pinia";
import { useToast } from "primevue";

export enum ToastSeverity {
    SUCCESS = "success",
    INFO = "info",
    WARN = "warn",
    ERROR = "error",
};

export const useToastStore = defineStore("toast", () => {
    const toast = useToast();

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

    return { showError, showInfo };
});
