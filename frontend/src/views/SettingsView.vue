<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";

import type { SupportedLocale } from "@/i18n";
import type { PasskeyCredential } from "@/services/auth/types";

import ChangelogModal from "@/components/changelog/ChangelogModal.vue";
import { usePasskeyRegistration } from "@/composables/usePasskeyRegistration";
import { SUPPORTED_LOCALES } from "@/i18n";
import { authService } from "@/services/auth";
import { versionService } from "@/services/version";
import { checkPasskeySupport } from "@/services/webauthn";
import { useAuthStore } from "@/stores/auth";
import { useSettingsStore } from "@/stores/settings";
import { useToast } from "@/stores/toast";
import { glassButtonsStyles } from "@/utils/pts/glassButtons";

const settingsStore = useSettingsStore();
const authStore = useAuthStore();
const toast = useToast();
const { t, locale } = useI18n();
const { addPasskeyToExistingAccount, isRegistering: addingPasskey } = usePasskeyRegistration();

// Form fields
const language = ref("");
const currency = ref("");
const alarmSet = ref(false);
const alarmTime = ref("");
const nightMode = ref(false);

// Passkey management
const passkeys = ref<PasskeyCredential[]>([]);
const loadingPasskeys = ref(false);
const removingPasskeyId = ref<string | null>(null);
const passkeySupported = ref(false);
const showAddPasskeyDialog = ref(false);
const deviceName = ref("");

// Version and changelog
const frontendVersion = ref("");
const showChangelog = ref(false);

// Load frontend version and passkeys
onMounted(async () => {
  frontendVersion.value = versionService.getFrontendVersion();
  await settingsStore.loadSettings();
  syncFormFields(settingsStore.settings);
  
  // Check passkey support
  const support = await checkPasskeySupport();
  passkeySupported.value = support.available;
  
  // Load passkeys
  await loadPasskeys();
});

// Sync form fields with store settings
function syncFormFields(newSettings: typeof settingsStore.settings) {
  if (newSettings) {
    language.value = newSettings.language;
    currency.value = newSettings.currency;
    alarmSet.value = newSettings.alarm_set;
    alarmTime.value = newSettings.alarm_time;
    nightMode.value = newSettings.night_mode;

    // Update i18n locale when settings are loaded, validate it's supported
    if (SUPPORTED_LOCALES.includes(newSettings.language as SupportedLocale)) {
      locale.value = newSettings.language;
    }
  }
}

// Watch for settings changes from store
watch(() => settingsStore.settings, syncFormFields);

// Available options - using computed to make them reactive to language changes
const languageOptions = computed(() => [
  { label: t("languages.en"), value: "en" },
  { label: t("languages.es"), value: "es" },
  { label: t("languages.fr"), value: "fr" },
  { label: t("languages.de"), value: "de" },
  { label: t("languages.ro"), value: "ro" },
]);

const currencyOptions = computed(() => [
  { label: t("currencies.usd"), value: "usd" },
  { label: t("currencies.eur"), value: "eur" },
  { label: t("currencies.ron"), value: "ron" },
]);

async function handleSave() {
  await settingsStore.updateSettings({
    language: language.value,
    currency: currency.value,
    alarm_set: alarmSet.value,
    alarm_time: alarmTime.value,
    alarm_offset_minutes: getLocaleToUtcOffsetMinutes(),
    night_mode: nightMode.value,
  });
  // Update locale immediately after saving settings to avoid waiting for watcher
  locale.value = language.value;
}

function handleLogout() {
  authStore.logout();
}

function openChangelog() {
  showChangelog.value = true;
}

function getLocaleToUtcOffsetMinutes(): number {
  const now = new Date();
  return -now.getTimezoneOffset();
}

// Passkey management functions
async function loadPasskeys() {
  loadingPasskeys.value = true;
  try {
    const response = await authService.passkeyList();
    if (response.status === 200 && response.data) {
      passkeys.value = response.data;
    } else {
      toast.error(t("settings.failed_to_load"));
    }
  } catch (error) {
    console.error("Failed to load passkeys:", error);
    toast.error(t("settings.failed_to_load"));
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
    toast.error(t("settings.enter_device_name"));
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
      toast.success(t("translation_keys.PASSKEY_REMOVED_SUCCESSFULLY"));
      await loadPasskeys();
    } else {
      const errorKey = response.data?.translation_key || "SOMETHING_WENT_WRONG";
      toast.error(t(`translation_keys.${errorKey}`));
    }
  } catch (error) {
    console.error("Failed to remove passkey:", error);
    toast.error(t("translation_keys.SOMETHING_WENT_WRONG"));
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
  <div class="flex items-center justify-center min-h-screen p-4 bg-gradient-to-b from-blue-300 to-blue-500">
    <Card class="w-full max-w-2xl backdrop-blur-2xl! bg-transparent! border! border-white/80! shadow-2xl!">
      <template #title>
        <div class="flex items-center gap-3 text-white">
          <i class="pi pi-cog text-2xl"></i>
          <span class="text-2xl font-medium">{{ t("settings.title") }}</span>
        </div>
      </template>
      <template #content>
        <div v-if="settingsStore.loading" class="flex justify-center py-8 text-white">
          <ProgressSpinner style="width: 50px; height: 50px" strokeWidth="4" fill="transparent"
            animationDuration="1s" />
        </div>
        <div v-else class="flex flex-col gap-6">
          <!-- Language Selection -->
          <div class="flex flex-col gap-2 text-white">
            <label for="language" class="font-medium">
              <i class="pi pi-globe mr-2"></i> {{ t("settings.language") }}
            </label>
            <Select id="language" v-model="language" :options="languageOptions" optionLabel="label" optionValue="value"
              :placeholder="t('settings.select_language')" class="w-full bg-transparent! border-white!" :pt="{
                label: {
                  class: 'text-white/90!',
                },
                overlay: {
                  class: 'bg-transparent! border-white! backdrop-blur-xs!',
                },
                option: {
                  class: 'text-white/90! bg-transparent! hover:bg-white/10!',
                },
              }">
              <template #dropdownicon>
                <i class="pi pi-chevron-down text-white" />
              </template>
            </Select>
          </div>

          <!-- Currency Selection -->
          <div class="flex flex-col gap-2 text-white">
            <label for="currency" class="font-medium">
              <i class="pi pi-dollar mr-2"></i> {{ t("settings.currency") }}
            </label>
            <Select id="currency" v-model="currency" :options="currencyOptions" optionLabel="label" optionValue="value"
              :placeholder="t('settings.select_currency')" class="w-full bg-transparent! border-white!" :pt="{
                label: {
                  class: 'text-white/90!',
                },
                overlay: {
                  class: 'bg-transparent! border-white! backdrop-blur-xs!',
                },
                option: {
                  class: 'text-white/90! bg-transparent! hover:bg-white/10!',
                },
              }">
              <template #dropdownicon>
                <i class="pi pi-chevron-down text-white" />
              </template>
            </Select>
          </div>

          <!-- Night Mode Toggle -->
          <div class="flex items-center justify-between">
            <label for="nightMode" class="text-white/90 font-medium">
              <i class="pi pi-moon mr-2"></i> {{ t("settings.night_mode") }}
            </label>
            <ToggleSwitch id="nightMode" v-model="nightMode" :pt="{
              slider: {
                class: 'bg-white/10!',
              },
              handle: {
                class: nightMode ? 'bg-black!' : 'bg-white!',
              },
            }" />
          </div>

          <!-- Alarm Settings Section -->
          <div class="flex flex-col gap-4">
            <div class="flex items-center justify-between">
              <label for="alarmSet" class="text-white/90 font-medium">
                <i class="pi pi-bell mr-2"></i> {{ t("settings.enable_alarm") }}
              </label>
              <ToggleSwitch id="alarmSet" v-model="alarmSet" :pt="{
                slider: {
                  class: 'bg-white/10!',
                },
                handle: {
                  class: alarmSet ? 'bg-black!' : 'bg-white!',
                },
              }" />
            </div>

            <div v-if="alarmSet" class="flex flex-col gap-4 ml-6">
              <!-- Alarm Time -->
              <div class="flex flex-col gap-2">
                <label for="alarmTime" class="text-white/90 font-medium">
                  <i class="pi pi-clock mr-2"></i> {{ t("settings.alarm_time") }}
                </label>
                <InputText id="alarmTime" v-model="alarmTime" type="time"
                  class="w-full bg-transparent! border-white! text-white!" />
              </div>
            </div>
          </div>

          <!-- Passkey Management Section -->
          <div v-if="passkeySupported" class="flex flex-col gap-4">
            <div class="flex items-center justify-between">
              <div class="flex flex-col">
                <label class="text-white/90 font-medium">
                  <i class="pi pi-key mr-2"></i> {{ t("settings.passkeys") }}
                </label>
                <span class="text-white/60 text-sm">{{ t("settings.passkeys_description") }}</span>
              </div>
              <Button @click="openAddPasskeyDialog" :label="t('settings.add_passkey')" icon="pi pi-plus"
                size="small" class="!rounded-3xl text-white! hover:text-blue-600!"
                :pt="glassButtonsStyles.selectedButtonPt" />
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
                <Button @click="handleRemovePasskey(passkey.id)" :label="t('settings.remove_passkey')"
                  icon="pi pi-trash" severity="danger" size="small"
                  :loading="removingPasskeyId === passkey.id" class="!rounded-3xl" />
              </div>
            </div>
          </div>

          <!-- Buttons and Version Row -->
          <div class="flex justify-between items-center mt-4">
            <Button @click="handleLogout" :label="t('settings.logout')" icon="pi pi-sign-out"
              class="!rounded-3xl text-white! hover:text-blue-600!" :pt="glassButtonsStyles.selectedButtonPt" />

            <button @click="openChangelog"
              class="text-white hover:text-white/40 text-sm transition-colors cursor-pointer"
              :title="t('settings.click_to_view_changelog')">
              {{ t('settings.version') }}: v{{ frontendVersion }}
            </button>

            <Button @click="handleSave" :label="t('settings.save_settings')" icon="pi pi-save"
              :loading="settingsStore.loading" class="!rounded-3xl text-white! hover:text-blue-600!"
              :pt="glassButtonsStyles.selectedButtonPt" />
          </div>
        </div>
      </template>
    </Card>

    <!-- Changelog Modal -->
    <ChangelogModal v-model:visible="showChangelog" />

    <!-- Add Passkey Dialog -->
    <Dialog v-model:visible="showAddPasskeyDialog" :header="t('settings.add_passkey')" :modal="true"
      class="w-full max-w-md backdrop-blur-2xl! bg-transparent! border! border-white/80! shadow-2xl!"
      :pt="{
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
          <Button :label="t('auth.forgot_password.go_back')" @click="showAddPasskeyDialog = false"
            severity="secondary" class="!rounded-3xl" />
          <Button :label="t('settings.add_passkey')" @click="handleAddPasskey" :loading="addingPasskey"
            icon="pi pi-plus" class="!rounded-3xl text-white! hover:text-blue-600!"
            :pt="glassButtonsStyles.selectedButtonPt" />
        </div>
      </template>
    </Dialog>
  </div>
</template>
