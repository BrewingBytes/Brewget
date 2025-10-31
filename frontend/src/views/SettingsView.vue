<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";

import { useSettingsStore } from "@/stores/settings";
import { glassButtonsStyles } from "@/utils/pts/glassButtons";

const settingsStore = useSettingsStore();
const { t, locale } = useI18n();

// Form fields
const language = ref("");
const currency = ref("");
const alarmSet = ref(false);
const alarmTime = ref("");
const nightMode = ref(false);

// Sync form fields with store settings
function syncFormFields(newSettings: typeof settingsStore.settings) {
  if (newSettings) {
    language.value = newSettings.language;
    currency.value = newSettings.currency;
    alarmSet.value = newSettings.alarm_set;
    alarmTime.value = newSettings.alarm_time;
    nightMode.value = newSettings.night_mode;

    // Update i18n locale when settings are loaded
    locale.value = newSettings.language;
  }
}

// Load settings on mount
onMounted(async () => {
  await settingsStore.loadSettings();
  syncFormFields(settingsStore.settings);
});

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
  const success = await settingsStore.updateSettings({
    language: language.value,
    currency: currency.value,
    alarm_set: alarmSet.value,
    alarm_time: alarmTime.value,
    alarm_offset_minutes: getLocaleToUtcOffsetMinutes(),
    night_mode: nightMode.value,
  });

  // Update i18n locale if save was successful
  if (success) {
    locale.value = language.value;
  }
}

function getLocaleToUtcOffsetMinutes(): number {
  const now = new Date();
  return -now.getTimezoneOffset();
}
</script>

<template>
  <div
    class="flex items-center justify-center min-h-screen p-4 bg-gradient-to-b from-blue-300 to-blue-500"
  >
    <Card
      class="w-full max-w-2xl backdrop-blur-2xl! bg-transparent! border! border-yellow/20! shadow-2xl!"
    >
      <template #title>
        <div class="flex items-center gap-3">
          <i class="pi pi-cog text-2xl"></i>
          <span class="text-2xl font-medium">{{ t("settings.title") }}</span>
        </div>
      </template>
      <template #content>
        <div v-if="settingsStore.loading" class="flex justify-center py-8">
          <ProgressSpinner
            style="width: 50px; height: 50px"
            strokeWidth="4"
            fill="transparent"
            animationDuration="1s"
          />
        </div>
        <div v-else class="flex flex-col gap-6">
          <!-- Language Selection -->
          <div class="flex flex-col gap-2">
            <label for="language" class="font-medium">
              <i class="pi pi-globe mr-2"></i>{{ t("settings.language") }}
            </label>
            <Select
              id="language"
              v-model="language"
              :options="languageOptions"
              optionLabel="label"
              optionValue="value"
              :placeholder="t('settings.select_language')"
              class="w-full bg-transparent! border-white!"
              :pt="{
                overlay: {
                  class: 'bg-transparent! border-white! backdrop-blur-xs!',
                },
                option: {
                  class: 'text-white/90 hover:bg-white/10!',
                },
              }"
            >
              <template #dropdownicon>
                <i class="pi pi-chevron-down text-white" />
              </template>
            </Select>
          </div>

          <!-- Currency Selection -->
          <div class="flex flex-col gap-2">
            <label for="currency" class="font-medium">
              <i class="pi pi-dollar mr-2"></i>{{ t("settings.currency") }}
            </label>
            <Select
              id="currency"
              v-model="currency"
              :options="currencyOptions"
              optionLabel="label"
              optionValue="value"
              :placeholder="t('settings.select_currency')"
              class="w-full bg-transparent! border-white!"
              :pt="{
                overlay: {
                  class: 'bg-transparent! border-white! backdrop-blur-xs!',
                },
                option: {
                  class: 'text-white/90 hover:bg-white/10!',
                },
              }"
            >
              <template #dropdownicon>
                <i class="pi pi-chevron-down text-white" />
              </template>
            </Select>
          </div>

          <!-- Night Mode Toggle -->
          <div class="flex items-center justify-between">
            <label for="nightMode" class="text-white/90 font-medium">
              <i class="pi pi-moon mr-2"></i>{{ t("settings.night_mode") }}
            </label>
            <ToggleSwitch
              id="nightMode"
              v-model="nightMode"
              :pt="{
                slider: {
                  class: 'bg-white/10!',
                },
                handle: {
                  class: nightMode ? 'bg-black!' : 'bg-white!',
                },
              }"
            />
          </div>

          <!-- Alarm Settings Section -->
          <div class="flex flex-col gap-4">
            <div class="flex items-center justify-between">
              <label for="alarmSet" class="text-white/90 font-medium">
                <i class="pi pi-bell mr-2"></i>{{ t("settings.enable_alarm") }}
              </label>
              <ToggleSwitch
                id="alarmSet"
                v-model="alarmSet"
                :pt="{
                  slider: {
                    class: 'bg-white/10!',
                  },
                  handle: {
                    class: alarmSet ? 'bg-black!' : 'bg-white!',
                  },
                }"
              />
            </div>

            <div v-if="alarmSet" class="flex flex-col gap-4 ml-6">
              <!-- Alarm Time -->
              <div class="flex flex-col gap-2">
                <label for="alarmTime" class="text-white/90 font-medium">
                  <i class="pi pi-clock mr-2"></i>{{ t("settings.alarm_time") }}
                </label>
                <InputText
                  id="alarmTime"
                  v-model="alarmTime"
                  type="time"
                  class="w-full bg-transparent! border-white!"
                />
              </div>
            </div>
          </div>

          <!-- Save Button -->
          <div class="flex justify-end mt-4">
            <Button
              @click="handleSave"
              :label="t('settings.save_settings')"
              icon="pi pi-save"
              :loading="settingsStore.loading"
              class="!rounded-3xl text-black! hover:text-blue-600!"
              :pt="glassButtonsStyles.selectedButtonPt"
            />
          </div>
        </div>
      </template>
    </Card>
  </div>
</template>
