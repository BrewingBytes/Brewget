<script setup lang="ts">
import { onMounted, ref, watch } from "vue";

import { useSettingsStore } from "@/stores/settings";
import { glassButtonsStyles } from "@/utils/pts/glassButtons";

const settingsStore = useSettingsStore();

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
  }
}

// Load settings on mount
onMounted(async () => {
  await settingsStore.loadSettings();
  syncFormFields(settingsStore.settings);
});

// Watch for settings changes from store
watch(() => settingsStore.settings, syncFormFields);

// Available options
const languageOptions = [
  { label: "English", value: "en" },
  { label: "Español", value: "es" },
  { label: "Français", value: "fr" },
  { label: "Deutsch", value: "de" },
];

const currencyOptions = [
  { label: "USD ($)", value: "usd" },
  { label: "EUR (€)", value: "eur" },
  { label: "RON (lei)", value: "ron" },
];

async function handleSave() {
  await settingsStore.updateSettings({
    language: language.value,
    currency: currency.value,
    alarm_set: alarmSet.value,
    alarm_time: alarmTime.value,
    alarm_offset_minutes: getLocaleToUtcOffsetMinutes(),
    night_mode: nightMode.value,
  });
}

function getLocaleToUtcOffsetMinutes(): number {
  const now = new Date();
  return -now.getTimezoneOffset();
}
</script>

<template>
  <div class="flex items-center justify-center min-h-screen p-4 bg-gradient-to-b from-blue-300 to-blue-500">
    <Card class="w-full max-w-2xl backdrop-blur-2xl! bg-transparent! border! border-yellow/20! shadow-2xl!">
      <template #title>
        <div class="flex items-center gap-3">
          <i class="pi pi-cog text-2xl"></i>
          <span class="text-2xl font-medium">User Settings</span>
        </div>
      </template>
      <template #content>
        <div v-if="settingsStore.loading" class="flex justify-center py-8">
          <ProgressSpinner style="width: 50px; height: 50px" strokeWidth="4" fill="transparent"
            animationDuration="1s" />
        </div>
        <div v-else class="flex flex-col gap-6">
          <!-- Language Selection -->
          <div class="flex flex-col gap-2">
            <label for="language" class="font-medium">
              <i class="pi pi-globe mr-2"></i>Language
            </label>
            <Select id="language" v-model="language" :options="languageOptions" optionLabel="label" optionValue="value"
              placeholder="Select a language" class="w-full bg-transparent! border-white!" :pt="{
                overlay: {
                  class: 'bg-transparent! border-white! backdrop-blur-xs!',
                },
                option: {
                  class: 'text-white/90 hover:bg-white/10!',
                },
              }">
              <template #dropdownicon>
                <i class="pi pi-chevron-down text-white" />
              </template>
            </Select>
          </div>

          <!-- Currency Selection -->
          <div class="flex flex-col gap-2">
            <label for="currency" class="font-medium">
              <i class="pi pi-dollar mr-2"></i>Currency
            </label>
            <Select id="currency" v-model="currency" :options="currencyOptions" optionLabel="label" optionValue="value"
              placeholder="Select a currency" class="w-full bg-transparent! border-white!" :pt="{
                overlay: {
                  class: 'bg-transparent! border-white! backdrop-blur-xs!',
                },
                option: {
                  class: 'text-white/90 hover:bg-white/10!',
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
              <i class="pi pi-moon mr-2"></i>Night Mode
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
                <i class="pi pi-bell mr-2"></i>Enable Alarm
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
                  <i class="pi pi-clock mr-2"></i>Alarm Time
                </label>
                <InputText id="alarmTime" v-model="alarmTime" type="time"
                  class="w-full bg-transparent! border-white!" />
              </div>

            </div>
          </div>

          <!-- Save Button -->
          <div class="flex justify-end mt-4">
            <Button @click="handleSave" label="Save Settings" icon="pi pi-save" :loading="settingsStore.loading"
              class="!rounded-3xl text-black! hover:text-blue-600!" :pt="glassButtonsStyles.selectedButtonPt" />
          </div>
        </div>
      </template>
    </Card>
  </div>
</template>
