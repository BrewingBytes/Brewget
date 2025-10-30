<script setup lang="ts">
import { useCookieConsentStore } from "@/stores/cookieConsent";

const cookieConsentStore = useCookieConsentStore();

function handleAccept(): void {
  cookieConsentStore.acceptCookies();
}
</script>

<template>
  <div v-if="!cookieConsentStore.hasAccepted" class="cookie-consent-overlay">
    <Dialog
      :visible="true"
      modal
      :closable="false"
      :draggable="false"
      header="Cookie Consent"
      class="cookie-consent-dialog"
    >
      <p class="mb-4">
        We use cookies to enhance your experience on our website. By continuing to use this site,
        you consent to our use of cookies.
      </p>
      <template #footer>
        <Button label="Accept" icon="pi pi-check" @click="handleAccept" autofocus />
      </template>
    </Dialog>
  </div>
</template>

<style scoped>
.cookie-consent-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  z-index: 9999;
  pointer-events: none;
}

.cookie-consent-dialog {
  pointer-events: auto;
}
</style>
