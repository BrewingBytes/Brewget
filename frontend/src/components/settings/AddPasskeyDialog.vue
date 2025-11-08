<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";

import GlassDialog from "@/components/glass/GlassDialog.vue";
import GlassInput from "@/components/glass/GlassInput.vue";
import { glassButtonsStyles } from "@/utils/pts/glassButtons";

interface Props {
  visible: boolean;
  loading: boolean;
}

interface Emits {
  (event: "update:visible", value: boolean): void;
  (event: "add", deviceName: string): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();
const { t } = useI18n();

const deviceName = ref("");

const handleAdd = () => {
  emit("add", deviceName.value);
};

const handleVisibilityChange = (value: boolean) => {
  if (!value) {
    deviceName.value = "";
  }
  emit("update:visible", value);
};
</script>

<template>
  <GlassDialog :visible="visible" @update:visible="handleVisibilityChange" :header="t('settings.add_passkey')"
    :style="{ width: '90vw', maxWidth: '500px' }">
    <div class="flex flex-col gap-4">
      <div class="flex flex-col gap-2">
        <label for="deviceName" class="text-white/90 font-medium">
          {{ t("settings.passkey_device_name") }}
        </label>
        <GlassInput id="deviceName" v-model="deviceName" />
      </div>
    </div>
    <template #footer>
      <div class="flex justify-end gap-2">
        <Button :label="t('auth.forgot_password.go_back')" @click="emit('update:visible', false)" severity="secondary"
          class="!rounded-3xl" />
        <Button :label="t('settings.add_passkey')" @click="handleAdd" :loading="loading" icon="pi pi-plus"
          class="!rounded-3xl text-white! hover:text-blue-600!" :pt="glassButtonsStyles.selectedButtonPt" />
      </div>
    </template>
  </GlassDialog>
</template>
