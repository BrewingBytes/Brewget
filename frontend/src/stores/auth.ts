import { defineStore } from "pinia";
import { computed, ref } from "vue";

import { authService } from "@/services/auth";
import { ServerStatus } from "@/services/types";
import { useRouter } from "vue-router";

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
    await authService.activate(values);
    router.push("/login");
  }

  async function login(values: { username: string, password: string }): Promise<void> {
    const response = await authService.login(values);

    // If error fail
    if (response.status !== ServerStatus.NO_ERROR) {
      console.error("Failed to login");
    }

    // Set bearer token
    token.value = response.data.token;
    router.push("/");
  }

  async function register(values: { email: string, username: string, password: string }): Promise<boolean> {
    return (await authService.register(values)).status === ServerStatus.NO_ERROR;
  }

  async function forgotPassword(values: { email: string }): Promise<boolean> {
    return (await authService.forgotPassword(values)).status === ServerStatus.NO_ERROR;
  }

  async function changePassword(values: { id: string, password: string }): Promise<void> {
    await authService.changePassword(values);

    router.push("/login");
  }

  async function logout(): Promise<void> {
    await authService.logout();

    token.value = "";
    router.push("/login");
  }

  return { activate, bearerToken, changePassword, isAuthenticated, login, register, forgotPassword, logout };
}, {
  persist: true,
});
