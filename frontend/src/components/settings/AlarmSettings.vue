<script setup lang="ts">
import { useI18n } from "vue-i18n";

import GlassInput from "@/components/glass/GlassInput.vue";

interface Props {
  alarmSet: boolean;
  alarmTime: string;
}

interface Emits {
  (event: "update:alarmSet", value: boolean): void;
  (event: "update:alarmTime", value: string): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();
const { t } = useI18n();
</script>

<template>
  <div class="flex flex-col gap-4">
    <div class="flex items-center justify-between">
      <label for="alarmSet" class="text-white/90 font-medium">
        <i class="pi pi-bell mr-2"></i> {{ t("settings.enable_alarm") }}
      </label>
      <ToggleSwitch id="alarmSet" :modelValue="alarmSet" @update:modelValue="emit('update:alarmSet', $event)" :pt="{
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
        <GlassInput id="alarmTime" :modelValue="alarmTime" @update:modelValue="emit('update:alarmTime', $event)"
          type="time" />
      </div>
    </div>
  </div>
</template>
