<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import type { CreateWallet } from "@/services/transaction/types";
import { useToastStore } from "@/stores/toast";
import GlassDialog from "@/components/glass/GlassDialog.vue";
import GlassInput from "@/components/glass/GlassInput.vue";
import GlassDropdown from "@/components/glass/GlassDropdown.vue";
import GlassButton from "@/components/glass/GlassButton.vue";

interface Props {
  visible: boolean;
  loading: boolean;
}

interface Emits {
  (event: "update:visible", value: boolean): void;
  (event: "create", wallet: CreateWallet): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();
const { t } = useI18n();

const newWalletBalance = ref("0");
const newWallet = ref<CreateWallet>({
  name: "",
  balance: 0,
  currency: "USD",
  wallet_type: "Account",
});

const currencyOptions = ["USD", "EUR", "GBP", "CAD", "JPY", "RON"];
const walletTypeOptions = ["Account", "Savings", "Deposit", "CreditCard", "Loan"];

const validateBalanceInput = (event: Event) => {
  const input = event.target as HTMLInputElement;
  let value = input.value;

  // Remove any non-numeric characters except decimal point
  value = value.replace(/[^0-9.]/g, "");

  // Ensure only one decimal point
  const parts = value.split(".");
  if (parts.length > 2) {
    value = `${parts[0]}.${parts.slice(1).join("")}`;
  }

  // Limit to 2 decimal places
  if (parts.length === 2 && parts[1] && parts[1].length > 2) {
    value = `${parts[0]}.${parts[1].substring(0, 2)}`;
  }

  input.value = value;
  newWalletBalance.value = value;
};

const resetForm = () => {
  newWallet.value = {
    name: "",
    balance: 0,
    currency: "USD",
    wallet_type: "Account",
  };
  newWalletBalance.value = "0";
};

const handleCreate = () => {
  // Validate name is required
  if (!newWallet.value.name || newWallet.value.name.trim() === "") {
    useToastStore().showError(t("wallets.name_required"));
    return;
  }

  // Parse balance from string, default to 0 if empty
  const balance = newWalletBalance.value ? parseFloat(newWalletBalance.value) : 0;
  newWallet.value.balance = balance;

  emit("create", newWallet.value);
};

// Reset form when dialog opens
const handleVisibilityChange = (value: boolean) => {
  if (value) {
    resetForm();
  }
  emit("update:visible", value);
};

defineExpose({ resetForm });
</script>

<template>
  <GlassDialog :visible="visible" @update:visible="handleVisibilityChange" :header="t('wallets.create_wallet')">
    <div class="space-y-4">
      <div>
        <label class="block mb-2 text-white/90"><i class="pi pi-tag mr-2"></i>{{ t("wallets.wallet_name")
        }}</label>
        <GlassInput v-model="newWallet.name" />
      </div>
      <div>
        <label class="block mb-2 text-white/90"><i class="pi pi-dollar mr-2"></i>{{ t("wallets.initial_balance")
        }}</label>
        <GlassInput v-model="newWalletBalance" @input="validateBalanceInput" />
      </div>
      <div>
        <label class="block mb-2 text-white/90"><i class="pi pi-money-bill mr-2"></i>{{ t("wallets.currency")
        }}</label>
        <GlassDropdown v-model="newWallet.currency" :options="currencyOptions" />
      </div>
      <div>
        <label class="block mb-2 text-white/90"><i class="pi pi-wallet mr-2"></i>{{ t("wallets.wallet_type")
        }}</label>
        <GlassDropdown v-model="newWallet.wallet_type" :options="walletTypeOptions" />
      </div>
    </div>
    <template #footer>
      <GlassButton :label="t('settings.save_settings')" icon="pi pi-check" @click="handleCreate" :loading="loading" />
    </template>
  </GlassDialog>
</template>
