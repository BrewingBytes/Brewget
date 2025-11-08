<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { glassButtonsStyles } from "@/utils/pts/glassButtons";

interface Props {
  hasPasskey: boolean;
  loading: boolean;
}

interface Emits {
  (event: "add"): void;
  (event: "remove"): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();
const { t } = useI18n();
</script>

<template>
  <div class="flex items-center justify-between">
    <div class="flex flex-col">
      <label class="text-white/90 font-medium">
        <i class="pi pi-key mr-2"></i> {{ t("settings.passkey") }}
      </label>
    </div>
    <Button v-if="hasPasskey" @click="emit('remove')" :label="t('settings.remove_passkey')" icon="pi pi-trash"
      :loading="loading" class="!rounded-3xl text-white! hover:text-blue-600!"
      :pt="glassButtonsStyles.selectedButtonPt" />
    <Button v-else @click="emit('add')" :label="t('settings.add_passkey')" icon="pi pi-plus" :loading="loading"
      class="!rounded-3xl text-white! hover:text-blue-600!" :pt="glassButtonsStyles.selectedButtonPt" />
  </div>
</template>
