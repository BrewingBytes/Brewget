<script setup lang="ts">
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import type { Wallet, UpdateWallet } from "@/services/transaction/types";
import { useToastStore } from "@/stores/toast";
import GlassDialog from "@/components/glass/GlassDialog.vue";
import GlassInput from "@/components/glass/GlassInput.vue";
import GlassDropdown from "@/components/glass/GlassDropdown.vue";
import GlassButton from "@/components/glass/GlassButton.vue";

interface Props {
  visible: boolean;
  loading: boolean;
  wallet: Wallet | null;
}

interface Emits {
  (event: "update:visible", value: boolean): void;
  (event: "update", id: number, wallet: UpdateWallet): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();
const { t } = useI18n();

const editWallet = ref<UpdateWallet>({
  name: "",
  currency: "USD",
  wallet_type: "Account",
});

const currencyOptions = ["USD", "EUR", "GBP", "CAD", "JPY", "RON"];
const walletTypeOptions = ["Account", "Savings", "Deposit", "CreditCard", "Loan"];

// Watch for wallet changes to update the form
watch(
  () => props.wallet,
  (newWallet) => {
    if (newWallet) {
      editWallet.value = {
        name: newWallet.name,
        currency: newWallet.currency,
        wallet_type: newWallet.wallet_type || "Account",
      };
    }
  },
  { immediate: true },
);

const handleUpdate = () => {
  if (!props.wallet) return;

  // Validate name is required
  if (!editWallet.value.name || editWallet.value.name.trim() === "") {
    useToastStore().showError(t("wallets.name_required"));
    return;
  }

  emit("update", props.wallet.id, editWallet.value);
};
</script>

<template>
  <GlassDialog :visible="visible" @update:visible="emit('update:visible', $event)" :header="t('wallets.edit_wallet')">
    <div class="space-y-4">
      <div>
        <label class="block mb-2 text-white/90"><i class="pi pi-tag mr-2"></i>{{ t("wallets.wallet_name") }}</label>
        <GlassInput v-model="editWallet.name" :placeholder="t('wallets.enter_wallet_name')" />
      </div>
      <div>
        <label class="block mb-2 text-white/90"><i class="pi pi-money-bill mr-2"></i>{{ t("wallets.currency")
        }}</label>
        <GlassDropdown v-model="editWallet.currency" :options="currencyOptions" />
      </div>
      <div>
        <label class="block mb-2 text-white/90"><i class="pi pi-wallet mr-2"></i>{{ t("wallets.wallet_type")
        }}</label>
        <GlassDropdown v-model="editWallet.wallet_type" :options="walletTypeOptions" />
      </div>
    </div>
    <template #footer>
      <GlassButton :label="t('settings.save_settings')" icon="pi pi-check" @click="handleUpdate" :loading="loading" />
    </template>
  </GlassDialog>
</template>
