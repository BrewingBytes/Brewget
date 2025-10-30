<script setup lang="ts">
import { useCookieConsentStore } from "@/stores/cookieConsent";

const cookieConsentStore = useCookieConsentStore();

function handleAccept(): void {
  cookieConsentStore.acceptCookies();
}
</script>

<template>
  <Dialog
    :visible="!cookieConsentStore.hasAccepted"
    modal
    :closable="false"
    :draggable="false"
    header="Cookie Consent"
    aria-describedby="cookie-consent-message"
    position="bottom"
  >
    <p id="cookie-consent-message" class="mb-4">
      We use cookies to enhance your experience on our website. You must accept cookies to use this site.
    </p>
    <template #footer>
      <Button label="Accept" icon="pi pi-check" @click="handleAccept" autofocus />
    </template>
  </Dialog>
</template>
