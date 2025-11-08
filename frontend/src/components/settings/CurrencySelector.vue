<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";

import GlassDropdown from "@/components/glass/GlassDropdown.vue";

interface Props {
  modelValue: string;
}

interface Emits {
  (event: "update:modelValue", value: string): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();
const { t } = useI18n();

const currencyOptions = computed(() => [
  { label: t("currencies.usd"), value: "usd" },
  { label: t("currencies.eur"), value: "eur" },
  { label: t("currencies.ron"), value: "ron" },
]);
</script>

<template>
  <div class="flex flex-col gap-2 text-white">
    <label for="currency" class="font-medium">
      <i class="pi pi-dollar mr-2"></i> {{ t("settings.currency") }}
    </label>
    <GlassDropdown id="currency" :modelValue="modelValue" @update:modelValue="emit('update:modelValue', $event)"
      :options="currencyOptions" optionLabel="label" optionValue="value"
      :placeholder="t('settings.select_currency')" />
  </div>
</template>
