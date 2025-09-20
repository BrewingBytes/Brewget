import { defineStore } from "pinia";
import { computed, ref } from "vue";

export const useAuthStore = defineStore("auth", () => {
  const token = ref("");
  const isAuthenticated = computed(() => {
    if (token.value === "") {
      return false;
    }

    return true;
  });

  function login(values: { username: string, password: string }) { console.error(values); }
  function register(values: { email: string, username: string, password: string }) { console.error(values); }
  function forgotPassword(values: { email: string }) { console.error(values); }

  return { isAuthenticated, login, register, forgotPassword };
}, {
  persist: true,
});
