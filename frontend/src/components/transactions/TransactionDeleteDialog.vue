<script setup lang="ts">
import { useI18n } from "vue-i18n";

import type { Transaction } from "@/services/transaction/types";

import GlassDialog from "@/components/glass/GlassDialog.vue";

interface Props {
  visible: boolean;
  loading: boolean;
  transaction: Transaction | null;
}

interface Emits {
  (event: "update:visible", value: boolean): void;
  (event: "delete", id: string): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();
const { t } = useI18n();

const handleDelete = () => {
  if (props.transaction) {
    emit("delete", props.transaction.id);
  }
};
</script>

<template>
  <GlassDialog
    :visible="visible"
    @update:visible="emit('update:visible', $event)"
    :header="t('transactions.delete_transaction')"
  >
    <p class="text-white/90">{{ t("transactions.confirm_delete") }}</p>
    <template #footer>
      <Button
        :label="t('wallets.cancel')"
        icon="pi pi-times"
        text
        @click="emit('update:visible', false)"
        class="text-white! hover:bg-white/10!"
      />
      <Button
        :label="t('transactions.delete_transaction')"
        icon="pi pi-trash"
        severity="danger"
        @click="handleDelete"
        :loading="loading"
        class="bg-red-500/20! border-red-300! text-red-300! hover:bg-red-500/30!"
      />
    </template>
  </GlassDialog>
</template>
