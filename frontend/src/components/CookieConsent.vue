<script setup lang="ts">
import { useI18n } from "vue-i18n";

import { useCookieConsentStore } from "@/stores/cookieConsent";

const cookieConsentStore = useCookieConsentStore();
const { t } = useI18n();

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
    :header="t('cookie_consent.title')"
    aria-describedby="cookie-consent-message"
    position="bottom"
  >
    <p id="cookie-consent-message" class="mb-4">
      {{ t('cookie_consent.message') }}
    </p>
    <template #footer>
      <Button :label="t('cookie_consent.accept')" icon="pi pi-check" @click="handleAccept" autofocus />
    </template>
  </Dialog>
</template>
