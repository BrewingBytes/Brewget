import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { useRouter } from "vue-router";

import { ToastSeverity, useToastStore } from "./toast";

import { authService } from "@/services/auth";
import { type ErrorResponse, ServerStatus } from "@/services/types";

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
    }): Promise<void> {
      const response = await authService.login(values);

      // If error fail
      if (response.status !== ServerStatus.NO_ERROR) {
        const errorResponse = response as ErrorResponse;
        useToastStore().showTranslationKey(
          errorResponse.data.translation_key,
          ToastSeverity.ERROR,
        );
        return;
      }

      // Set bearer token
      token.value = response.data.token;
      router.push("/");
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

    return {
      token,
      activate,
      bearerToken,
      changePassword,
      isAuthenticated,
      login,
      register,
      forgotPassword,
      logout,
    };
  },
  {
    persist: true,
  },
);
