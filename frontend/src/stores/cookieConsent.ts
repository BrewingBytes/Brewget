import { defineStore } from "pinia";
import { ref } from "vue";

export const useCookieConsentStore = defineStore("cookieConsent", () => {
  const hasAccepted = ref(false);

  function acceptCookies(): void {
    hasAccepted.value = true;
  }

  return { hasAccepted, acceptCookies };
}, {
  persist: true,
});
