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

const languageOptions = computed(() => [
  { label: t("languages.en"), value: "en" },
  { label: t("languages.es"), value: "es" },
  { label: t("languages.fr"), value: "fr" },
  { label: t("languages.de"), value: "de" },
  { label: t("languages.ro"), value: "ro" },
]);
</script>

<template>
  <div class="flex flex-col gap-2 text-white">
    <label for="language" class="font-medium">
      <i class="pi pi-globe mr-2"></i> {{ t("settings.language") }}
    </label>
    <GlassDropdown id="language" :modelValue="modelValue" @update:modelValue="emit('update:modelValue', $event)"
      :options="languageOptions" optionLabel="label" optionValue="value"
      :placeholder="t('settings.select_language')" />
  </div>
</template>
