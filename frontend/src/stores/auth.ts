import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { useRouter } from "vue-router";

import { useSettingsStore } from "./settings";
import { ToastSeverity, useToastStore } from "./toast";

import type { SupportedLocale } from "@/i18n";

import { usePasskeyRegistration } from "@/composables/usePasskeyRegistration";
import i18n, { SUPPORTED_LOCALES } from "@/i18n";
import { authService } from "@/services/auth";
import { type ErrorResponse, ServerStatus } from "@/services/types";
import {
  assertionToJSON,
  authenticateWithPasskey,
} from "@/services/webauthn";

export const useAuthStore = defineStore(
  "auth",
  () => {
    const token = ref("");
    const router = useRouter();

    const isAuthenticated = computed(() => {
      if (token.value === "") {
        return false;
      }

      return true;
    });

    const bearerToken = computed(() => `Bearer ${token.value}`);

    async function activate(values: { id: string }): Promise<void> {
      const response = await authService.activate(values);

      if (response.status !== ServerStatus.NO_ERROR) {
        const errorResponse = response as ErrorResponse;
        useToastStore().showTranslationKey(
          errorResponse.data.translation_key,
          ToastSeverity.ERROR,
        );
      } else {
        useToastStore().showTranslationKey(
          response.data.translation_key,
          ToastSeverity.SUCCESS,
        );
      }

      router.push("/login");
    }

    async function login(values: {
      username: string;
      password: string;
      captchaToken: string;
    }): Promise<boolean> {
      const response = await authService.login(values);

      // If error fail
      if (response.status !== ServerStatus.NO_ERROR) {
        const errorResponse = response as ErrorResponse;
        useToastStore().showTranslationKey(
          errorResponse.data.translation_key,
          ToastSeverity.ERROR,
        );
        return false;
      }

      // Set bearer token
      token.value = response.data.token;

      // Load user settings and apply language preference
      const settingsStore = useSettingsStore();
      settingsStore.loadSettings().then(() => {
        if (settingsStore.settings && SUPPORTED_LOCALES.includes(settingsStore.settings.language as SupportedLocale)) {
          i18n.global.locale.value = settingsStore.settings.language as SupportedLocale;
        }
      });

      router.push("/");

      return true;
    }

    async function register(values: {
      email: string;
      username: string;
      password: string;
      captchaToken: string;
    }): Promise<boolean> {
      const response = await authService.register(values);
      if (response.status !== ServerStatus.NO_ERROR) {
        const errorResponse = response as ErrorResponse;
        useToastStore().showTranslationKey(
          errorResponse.data.translation_key,
          ToastSeverity.ERROR,
        );
        return false;
      }

      useToastStore().showTranslationKey(
        response.data.translation_key,
        ToastSeverity.SUCCESS,
      );

      return true;
    }

    async function forgotPassword(values: {
      email: string;
      captchaToken: string;
    }): Promise<boolean> {
      const response = await authService.forgotPassword(values);
      if (response.status !== ServerStatus.NO_ERROR) {
        const errorResponse = response as ErrorResponse;
        useToastStore().showTranslationKey(
          errorResponse.data.translation_key,
          ToastSeverity.ERROR,
        );
        return false;
      }

      useToastStore().showTranslationKey(
        response.data.translation_key,
        ToastSeverity.INFO,
      );

      return true;
    }

    async function changePassword(values: {
      id: string;
      password: string;
    }): Promise<void> {
      const response = await authService.changePassword(values);
      if (response.status !== ServerStatus.NO_ERROR) {
        const errorResponse = response as ErrorResponse;
        useToastStore().showTranslationKey(
          errorResponse.data.translation_key,
          ToastSeverity.ERROR,
        );
        return;
      }

      useToastStore().showTranslationKey(
        response.data.translation_key,
        ToastSeverity.SUCCESS,
      );

      router.push("/login");
    }

    async function logout(): Promise<void> {
      await authService.logout();

      token.value = "";
      router.push("/login");
    }

    async function verifyToken(): Promise<boolean> {
      if (token.value === "") {
        return false;
      }

      const response = await authService.verify();

      // If token is invalid or expired, the axios interceptor will handle logout
      // and show the appropriate message
      if (response.status !== ServerStatus.NO_ERROR) {
        return false;
      }

      return true;
    }

    async function registerWithPasskey(values: {
      email: string;
      username: string;
      captchaToken: string;
      deviceName?: string;
    }): Promise<boolean> {
      const { registerPasskeyForNewAccount } = usePasskeyRegistration();
      return await registerPasskeyForNewAccount({
        username: values.username,
        email: values.email,
        captchaToken: values.captchaToken,
        deviceName: values.deviceName,
      });
    }

    async function loginWithPasskey(values: {
      username: string;
      captchaToken: string;
    }): Promise<boolean> {
      try {
        // Start passkey login
        const startResponse = await authService.passkeyLoginStart({
          username: values.username,
          captchaToken: values.captchaToken,
        });

        if (startResponse.status !== ServerStatus.NO_ERROR) {
          const errorResponse = startResponse as ErrorResponse;
          useToastStore().showTranslationKey(
            errorResponse.data.translation_key,
            ToastSeverity.ERROR,
          );
          return false;
        }

        // Authenticate with passkey
        const credential = await authenticateWithPasskey(
          startResponse.data.request_options,
        );
        const assertionJSON = assertionToJSON(credential);

        // Complete passkey login
        const finishResponse = await authService.passkeyLoginFinish({
          username: values.username,
          credential: assertionJSON,
        });

        if (finishResponse.status !== ServerStatus.NO_ERROR) {
          const errorResponse = finishResponse as ErrorResponse;
          useToastStore().showTranslationKey(
            errorResponse.data.translation_key,
            ToastSeverity.ERROR,
          );
          return false;
        }

        // Set bearer token
        token.value = finishResponse.data.token;

        // Load user settings and apply language preference
        const settingsStore = useSettingsStore();
        settingsStore.loadSettings().then(() => {
          if (
            settingsStore.settings &&
            SUPPORTED_LOCALES.includes(settingsStore.settings.language as SupportedLocale)
          ) {
            i18n.global.locale.value =
              settingsStore.settings.language as SupportedLocale;
          }
        });

        router.push("/");

        return true;
      } catch (error) {
        console.error("Passkey login error:", error);
        useToastStore().showTranslationKey(
          "PASSKEY_AUTHENTICATION_FAILED",
          ToastSeverity.ERROR,
        );
        return false;
      }
    }

    return {
      token,
      activate,
      bearerToken,
      changePassword,
      isAuthenticated,
      login,
      loginWithPasskey,
      register,
      registerWithPasskey,
      forgotPassword,
      logout,
      verifyToken,
    };
  },
  {
    persist: true,
  },
);
