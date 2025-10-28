import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { useRouter } from "vue-router";

import { useToastStore } from "./toast";

import { authService } from "@/services/auth";
import { type ErrorResponse, ServerStatus } from "@/services/types";

export const useAuthStore = defineStore("auth", () => {
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
      useToastStore().showError("Activation link is invalid.");
    }

    router.push("/login");
  }

  async function login(values: { username: string, password: string, captchaToken: string }): Promise<void> {
    const response = await authService.login(values);

    // If error fail
    if (response.status !== ServerStatus.NO_ERROR) {
      useToastStore().showError("Username or password is invalid.");
      return;
    }

    // Set bearer token
    token.value = response.data.token;
    router.push("/");
  }

  async function register(values: { email: string, username: string, password: string, captchaToken: string }): Promise<boolean> {
    const response = await authService.register(values);
    if (response.status !== ServerStatus.NO_ERROR) {
      useToastStore().showError((response as ErrorResponse).data.message);
      return false;
    }

    return true;
  }

  async function forgotPassword(values: { email: string, captchaToken: string }): Promise<boolean> {
    const response = await authService.forgotPassword(values);
    if (response.status !== ServerStatus.NO_ERROR) {
      useToastStore().showError((response as ErrorResponse).data.message);
      return false;
    }

    useToastStore().showInfo(response.data.message);
    return true;
  }

  async function changePassword(values: { id: string, password: string }): Promise<void> {
    const response = await authService.changePassword(values);
    if (response.status === ServerStatus.UNPROCESSABLE_CONTENT) {
      useToastStore().showError("Activation link is invalid.");
    } else if (response.status === ServerStatus.BAD_REQUEST) {
      useToastStore().showError((response as ErrorResponse).data.message);
      return;
    }

    useToastStore().showInfo("Account password has been changed.");
    router.push("/login");
  }

  async function logout(): Promise<void> {
    await authService.logout();

    token.value = "";
    router.push("/login");
  }

  return { token, activate, bearerToken, changePassword, isAuthenticated, login, register, forgotPassword, logout };
}, {
  persist: true,
});
