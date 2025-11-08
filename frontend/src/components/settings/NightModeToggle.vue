<script setup lang="ts">
import { useI18n } from "vue-i18n";

interface Props {
  modelValue: boolean;
}

interface Emits {
  (event: "update:modelValue", value: boolean): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();
const { t } = useI18n();
</script>

<template>
  <div class="flex items-center justify-between">
    <label for="nightMode" class="text-white/90 font-medium">
      <i class="pi pi-moon mr-2"></i> {{ t("settings.night_mode") }}
    </label>
    <ToggleSwitch id="nightMode" :modelValue="modelValue" @update:modelValue="emit('update:modelValue', $event)" :pt="{
      slider: {
        class: 'bg-white/10!',
      },
      handle: {
        class: modelValue ? 'bg-black!' : 'bg-white!',
      },
    }" />
  </div>
</template>
