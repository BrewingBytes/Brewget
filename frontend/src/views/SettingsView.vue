<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";

import type { SupportedLocale } from "@/i18n";

import AuthAuditModal from "@/components/AuthAuditModal.vue";
import ChangelogModal from "@/components/changelog/ChangelogModal.vue";
import GlassButton from "@/components/glass/GlassButton.vue";
import GlassCard from "@/components/glass/GlassCard.vue";
import AddPasskeyDialog from "@/components/settings/AddPasskeyDialog.vue";
import AlarmSettings from "@/components/settings/AlarmSettings.vue";
import CurrencySelector from "@/components/settings/CurrencySelector.vue";
import LanguageSelector from "@/components/settings/LanguageSelector.vue";
import NightModeToggle from "@/components/settings/NightModeToggle.vue";
import { SUPPORTED_LOCALES } from "@/i18n";
import { authService } from "@/services/auth";
import { versionService } from "@/services/version";
import { checkPasskeySupport } from "@/services/webauthn";
import { useAuthStore } from "@/stores/auth";
import { useSettingsStore } from "@/stores/settings";

const settingsStore = useSettingsStore();
const authStore = useAuthStore();
const { t, locale } = useI18n();

// Form fields
const language = ref("");
const currency = ref("");
const alarmSet = ref(false);
const alarmTime = ref("");
const nightMode = ref(false);

// Passkey management
const hasPasskey = ref(false);
const passkeySupported = ref(false);
const loadingPasskey = ref(false);
const showAddPasskeyDialog = ref(false);

// Version and changelog
const frontendVersion = ref("");
const showChangelog = ref(false);
const showAuthAudit = ref(false);

// Load frontend version
onMounted(async () => {
  frontendVersion.value = versionService.getFrontendVersion();
  await settingsStore.loadSettings();
  syncFormFields(settingsStore.settings);

  // Check passkey support
  const support = await checkPasskeySupport();
  passkeySupported.value = support.available;

  // Check if user has passkey
  if (passkeySupported.value) {
    await checkUserPasskey();
  }
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

function openAuthAudit() {
  showAuthAudit.value = true;
}

function getLocaleToUtcOffsetMinutes(): number {
  const now = new Date();
  return -now.getTimezoneOffset();
}

// Passkey management functions
async function checkUserPasskey() {
  loadingPasskey.value = true;
  try {
    const response = await authService.passkeyList();
    if (response.status === 200 && response.data) {
      hasPasskey.value = response.data.length > 0;
    }
  } catch (error) {
    console.error("Failed to check passkey:", error);
  } finally {
    loadingPasskey.value = false;
  }
}

async function handleAddPasskey(_deviceName: string) {
  // TODO: Implement passkey addition with WebAuthn
  console.log("Add passkey not yet implemented");
  showAddPasskeyDialog.value = false;
}

async function handleDeletePasskey() {
  loadingPasskey.value = true;
  try {
    const response = await authService.passkeyList();
    if (response.status === 200 && response.data && response.data.length > 0) {
      const passkeyId = response.data[0]?.id as string;
      const deleteResponse = await authService.passkeyRemove(passkeyId);
      if (deleteResponse.status === 200) {
        await checkUserPasskey();
      }
    }
  } catch (error) {
    console.error("Failed to remove passkey:", error);
  } finally {
    loadingPasskey.value = false;
  }
}
</script>

<template>
  <div class="flex items-center justify-center min-h-screen p-4 bg-gradient-to-b from-blue-300 to-blue-500">
    <GlassCard :title="t('settings.title')" icon="pi-cog" class="w-full max-w-2xl">
      <template #content>
        <div v-if="settingsStore.loading" class="flex justify-center py-8 text-white">
          <ProgressSpinner style="width: 50px; height: 50px" strokeWidth="4" fill="transparent"
            animationDuration="1s" />
        </div>
        <div v-else class="flex flex-col gap-6">
          <!-- Language Selection -->
          <LanguageSelector v-model="language" />

          <!-- Currency Selection -->
          <CurrencySelector v-model="currency" />

          <!-- Night Mode Toggle -->
          <NightModeToggle v-model="nightMode" />

          <!-- Alarm Settings -->
          <AlarmSettings v-model:alarmSet="alarmSet" v-model:alarmTime="alarmTime" />

          <!-- Passkey Management -->
          <div v-if="passkeySupported" class="flex items-center justify-between">
            <div class="flex flex-col">
              <label class="text-white/90 font-medium">
                <i class="pi pi-key mr-2"></i> {{ t("settings.passkey") }}
              </label>
            </div>
            <GlassButton v-if="hasPasskey" @click="handleDeletePasskey" :label="t('settings.remove_passkey')" icon="pi pi-trash"
              :loading="loadingPasskey" />
            <GlassButton v-else @click="showAddPasskeyDialog = true" :label="t('settings.add_passkey')"
              icon="pi pi-plus" :loading="loadingPasskey" />
          </div>

          <!-- AddPasskey Dialog -->
          <AddPasskeyDialog v-model:visible="showAddPasskeyDialog" :loading="loadingPasskey" @add="handleAddPasskey" />

          <!-- Auth Audit Button -->
          <div class="flex items-center justify-between">
            <div class="flex flex-col">
              <label class="text-white/90 font-medium">
                <i class="pi pi-history mr-2"></i> {{ t("settings.auth_activity") }}
              </label>
            </div>
            <GlassButton @click="openAuthAudit" :label="t('settings.view_auth_activity')" icon="pi pi-eye" />
          </div>

          <!-- Buttons and Version Row -->
          <div class="flex justify-between items-center mt-4">
            <GlassButton @click="handleLogout" :label="t('settings.logout')" icon="pi pi-sign-out" />

            <button @click="openChangelog"
              class="text-white hover:text-white/40 text-sm transition-colors cursor-pointer"
              :title="t('settings.click_to_view_changelog')">
              {{ t('settings.version') }}: v{{ frontendVersion }}
            </button>

            <GlassButton @click="handleSave" :label="t('settings.save_settings')" icon="pi pi-save"
              :loading="settingsStore.loading" />
          </div>
        </div>
      </template>
    </GlassCard>

    <!-- Changelog Modal -->
    <ChangelogModal v-model:visible="showChangelog" />

    <!-- Auth Audit Modal -->
    <AuthAuditModal v-model:visible="showAuthAudit" />
  </div>
</template>
