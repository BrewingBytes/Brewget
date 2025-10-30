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
      aria-describedby="cookie-consent-message"
    >
      <p id="cookie-consent-message" class="mb-4">
        We use cookies to enhance your experience on our website. You must accept cookies to use this site.
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
