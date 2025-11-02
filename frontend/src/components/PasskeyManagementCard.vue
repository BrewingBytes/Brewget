<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";

import type { PasskeyCredential } from "@/services/auth/types";

import { usePasskeyRegistration } from "@/composables/usePasskeyRegistration";
import { authService } from "@/services/auth";
import { checkPasskeySupport } from "@/services/webauthn";
import { useToastStore } from "@/stores/toast";
import { glassButtonsStyles } from "@/utils/pts/glassButtons";

const toast = useToastStore();
const { t, locale } = useI18n();
const { addPasskeyToExistingAccount, isRegistering: addingPasskey } = usePasskeyRegistration();

// Passkey management
const passkeys = ref<PasskeyCredential[]>([]);
const loadingPasskeys = ref(false);
const removingPasskeyId = ref<string | null>(null);
const passkeySupported = ref(false);
const showAddPasskeyDialog = ref(false);
const deviceName = ref("");

// Emit event to close the card
const emit = defineEmits<{
  close: [];
}>();

onMounted(async () => {
  // Check passkey support
  const support = await checkPasskeySupport();
  passkeySupported.value = support.available;

  // Load passkeys
  await loadPasskeys();
});

// Passkey management functions
async function loadPasskeys() {
  loadingPasskeys.value = true;
  try {
    const response = await authService.passkeyList();
    if (response.status === 200 && response.data) {
      passkeys.value = response.data;
    } else {
      toast.showError(t("settings.failed_to_load"));
    }
  } catch (error) {
    console.error("Failed to load passkeys:", error);
    toast.showError(t("settings.failed_to_load"));
  } finally {
    loadingPasskeys.value = false;
  }
}

function openAddPasskeyDialog() {
  deviceName.value = "";
  showAddPasskeyDialog.value = true;
}

async function handleAddPasskey() {
  if (!deviceName.value.trim()) {
    toast.showError(t("settings.enter_device_name"));
    return;
  }

  const success = await addPasskeyToExistingAccount(deviceName.value.trim());

  if (success) {
    showAddPasskeyDialog.value = false;
    await loadPasskeys();
  }
}

async function handleRemovePasskey(id: string) {
  removingPasskeyId.value = id;
  try {
    const response = await authService.passkeyRemove(id);
    if (response.status === 200) {
      toast.showSuccess(t("translation_keys.PASSKEY_REMOVED_SUCCESSFULLY"));
      await loadPasskeys();
    } else {
      const errorKey = response.data?.translation_key || "SOMETHING_WENT_WRONG";
      toast.showError(t(`translation_keys.${errorKey}`));
    }
  } catch (error) {
    console.error("Failed to remove passkey:", error);
    toast.showError(t("translation_keys.SOMETHING_WENT_WRONG"));
  } finally {
    removingPasskeyId.value = null;
  }
}

function formatDate(dateString: string | null): string {
  if (!dateString) return t("settings.passkey_never_used");
  const date = new Date(dateString);
  return date.toLocaleDateString(locale.value, {
    year: "numeric",
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
}
</script>

<template>
  <Card class="w-full max-w-2xl backdrop-blur-2xl! bg-transparent! border! border-white/80! shadow-2xl!">
    <template #title>
      <div class="flex items-center justify-between text-white">
        <div class="flex items-center gap-3">
          <i class="pi pi-key text-2xl"></i>
          <span class="text-2xl font-medium">{{ t("settings.passkeys") }}</span>
        </div>
        <Button icon="pi pi-times" @click="emit('close')" text rounded class="text-white! hover:bg-white/10!" />
      </div>
    </template>
    <template #content>
      <div v-if="!passkeySupported" class="text-white/80 text-center py-8">
        {{ t("translation_keys.PASSKEY_NOT_SUPPORTED") }}
      </div>
      <div v-else class="flex flex-col gap-4">
        <div class="flex items-center justify-between">
          <div class="flex flex-col">
            <span class="text-white/60 text-sm">{{ t("settings.passkeys_description") }}</span>
          </div>
          <Button @click="openAddPasskeyDialog" :label="t('settings.add_passkey')" icon="pi pi-plus" size="small"
            class="!rounded-3xl text-white! hover:text-blue-600!" :pt="glassButtonsStyles.selectedButtonPt" />
        </div>

        <!-- Passkeys List -->
        <div v-if="loadingPasskeys" class="flex justify-center py-4">
          <ProgressSpinner style="width: 30px; height: 30px" strokeWidth="4" fill="transparent"
            animationDuration="1s" />
        </div>
        <div v-else-if="passkeys.length === 0" class="text-white/60 text-center py-4">
          {{ t("settings.no_passkeys") }}
        </div>
        <div v-else class="flex flex-col gap-3">
          <div v-for="passkey in passkeys" :key="passkey.id"
            class="flex items-center justify-between p-4 bg-white/10 rounded-lg backdrop-blur-sm">
            <div class="flex flex-col gap-1">
              <span class="text-white font-medium">
                {{ passkey.device_name || t("settings.passkey_device_name") }}
              </span>
              <span class="text-white/60 text-sm">
                {{ t("settings.passkey_created") }}: {{ formatDate(passkey.created_at) }}
              </span>
              <span class="text-white/60 text-sm">
                {{ t("settings.passkey_last_used") }}: {{ formatDate(passkey.last_used_at) }}
              </span>
            </div>
            <Button @click="handleRemovePasskey(passkey.id)" :label="t('settings.remove_passkey')" icon="pi pi-trash"
              severity="danger" size="small" :loading="removingPasskeyId === passkey.id" class="!rounded-3xl" />
          </div>
        </div>
      </div>
    </template>
  </Card>

  <!-- Add Passkey Dialog -->
  <Dialog v-model:visible="showAddPasskeyDialog" :header="t('settings.add_passkey')" :modal="true"
    class="w-full max-w-md backdrop-blur-2xl! bg-transparent! border! border-white/80! shadow-2xl!" :pt="{
      root: {
        class: 'backdrop-blur-2xl! bg-white/10! border! border-white/80!',
      },
      header: {
        class: 'text-white! bg-transparent!',
      },
      content: {
        class: 'text-white! bg-transparent!',
      },
      footer: {
        class: 'bg-transparent!',
      },
    }">
    <div class="flex flex-col gap-4">
      <div class="flex flex-col gap-2">
        <label for="deviceName" class="text-white/90 font-medium">
          {{ t("settings.passkey_device_name") }}
        </label>
        <InputText id="deviceName" v-model="deviceName" :placeholder="t('settings.enter_device_name')"
          class="w-full bg-transparent! border-white! text-white!" />
      </div>
    </div>
    <template #footer>
      <div class="flex justify-end gap-2">
        <Button :label="t('auth.forgot_password.go_back')" @click="showAddPasskeyDialog = false" severity="secondary"
          class="!rounded-3xl" />
        <Button :label="t('settings.add_passkey')" @click="handleAddPasskey" :loading="addingPasskey" icon="pi pi-plus"
          class="!rounded-3xl text-white! hover:text-blue-600!" :pt="glassButtonsStyles.selectedButtonPt" />
      </div>
    </template>
  </Dialog>
</template>
