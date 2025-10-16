<script setup lang="ts">
import { onMounted, ref, watch } from "vue";

import { useSettingsStore } from "@/stores/settings";

const settingsStore = useSettingsStore();

// Form fields
const language = ref("");
const currency = ref("");
const alarmSet = ref(false);
const alarmTime = ref("");
const alarmOffsetMinutes = ref(0);
const nightMode = ref(false);

// Load settings on mount
onMounted(async () => {
  await settingsStore.loadSettings();

  // Populate form fields with loaded settings
  if (settingsStore.settings) {
    language.value = settingsStore.settings.language;
    currency.value = settingsStore.settings.currency;
    alarmSet.value = settingsStore.settings.alarm_set;
    alarmTime.value = settingsStore.settings.alarm_time;
    alarmOffsetMinutes.value = settingsStore.settings.alarm_offset_minutes;
    nightMode.value = settingsStore.settings.night_mode;
  }
});

// Watch for settings changes from store
watch(
  () => settingsStore.settings,
  (newSettings) => {
    if (newSettings) {
      language.value = newSettings.language;
      currency.value = newSettings.currency;
      alarmSet.value = newSettings.alarm_set;
      alarmTime.value = newSettings.alarm_time;
      alarmOffsetMinutes.value = newSettings.alarm_offset_minutes;
      nightMode.value = newSettings.night_mode;
    }
  },
);

// Available options
const languageOptions = [
  { label: "English", value: "en" },
  { label: "Español", value: "es" },
  { label: "Français", value: "fr" },
  { label: "Deutsch", value: "de" },
];

const currencyOptions = [
  { label: "USD ($)", value: "USD" },
  { label: "EUR (€)", value: "EUR" },
  { label: "GBP (£)", value: "GBP" },
  { label: "JPY (¥)", value: "JPY" },
];

async function handleSave() {
  await settingsStore.updateSettings({
    language: language.value,
    currency: currency.value,
    alarm_set: alarmSet.value,
    alarm_time: alarmTime.value,
    alarm_offset_minutes: alarmOffsetMinutes.value,
    night_mode: nightMode.value,
  });
}
</script>

<template>
  <div class="flex items-center justify-center min-h-screen p-4">
    <Card
      class="w-full max-w-2xl backdrop-blur-2xl bg-white/10 border border-white/20 shadow-2xl"
    >
      <template #title>
        <div class="flex items-center gap-3 text-white">
          <i class="pi pi-cog text-2xl"></i>
          <span class="text-2xl font-medium">User Settings</span>
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
            <label for="language" class="text-white/90 font-medium">
              <i class="pi pi-globe mr-2"></i>Language
            </label>
            <Select
              id="language"
              v-model="language"
              :options="languageOptions"
              optionLabel="label"
              optionValue="value"
              placeholder="Select a language"
              class="w-full"
            />
          </div>

          <!-- Currency Selection -->
          <div class="flex flex-col gap-2">
            <label for="currency" class="text-white/90 font-medium">
              <i class="pi pi-dollar mr-2"></i>Currency
            </label>
            <Select
              id="currency"
              v-model="currency"
              :options="currencyOptions"
              optionLabel="label"
              optionValue="value"
              placeholder="Select a currency"
              class="w-full"
            />
          </div>

          <!-- Night Mode Toggle -->
          <div class="flex items-center justify-between">
            <label for="nightMode" class="text-white/90 font-medium">
              <i class="pi pi-moon mr-2"></i>Night Mode
            </label>
            <ToggleSwitch id="nightMode" v-model="nightMode" />
          </div>

          <!-- Alarm Settings Section -->
          <Divider class="!border-white/20" />

          <div class="flex flex-col gap-4">
            <div class="flex items-center justify-between">
              <label for="alarmSet" class="text-white/90 font-medium">
                <i class="pi pi-bell mr-2"></i>Enable Alarm
              </label>
              <ToggleSwitch id="alarmSet" v-model="alarmSet" />
            </div>

            <div v-if="alarmSet" class="flex flex-col gap-4 ml-6">
              <!-- Alarm Time -->
              <div class="flex flex-col gap-2">
                <label for="alarmTime" class="text-white/90 font-medium">
                  <i class="pi pi-clock mr-2"></i>Alarm Time
                </label>
                <InputText
                  id="alarmTime"
                  v-model="alarmTime"
                  type="time"
                  class="w-full"
                />
              </div>

              <!-- Alarm Offset -->
              <div class="flex flex-col gap-2">
                <label for="alarmOffset" class="text-white/90 font-medium">
                  <i class="pi pi-stopwatch mr-2"></i>Alarm Offset (minutes)
                </label>
                <InputNumber
                  id="alarmOffset"
                  v-model="alarmOffsetMinutes"
                  :min="0"
                  :max="120"
                  showButtons
                  class="w-full"
                />
              </div>
            </div>
          </div>

          <!-- Save Button -->
          <div class="flex justify-end mt-4">
            <Button
              @click="handleSave"
              label="Save Settings"
              icon="pi pi-save"
              :loading="settingsStore.loading"
              class="!rounded-3xl !bg-blue-600 !border-blue-600 text-white hover:!bg-blue-700"
            />
          </div>
        </div>
      </template>
    </Card>
  </div>
</template>
