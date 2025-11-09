<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";

import type { CreateTransaction } from "@/services/transaction/types";

import GlassButton from "@/components/glass/GlassButton.vue";
import GlassDialog from "@/components/glass/GlassDialog.vue";
import GlassDropdown from "@/components/glass/GlassDropdown.vue";
import GlassInput from "@/components/glass/GlassInput.vue";
import { useToastStore } from "@/stores/toast";
import { useWalletStore } from "@/stores/wallet";

interface Props {
  visible: boolean;
  loading: boolean;
}

interface Emits {
  (event: "update:visible", value: boolean): void;
  (event: "create", transaction: CreateTransaction): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();
const { t } = useI18n();

const walletStore = useWalletStore();

const newTransactionAmount = ref("0");
const transactionDate = ref<Date>(new Date());
const newTransaction = ref<CreateTransaction>({
  wallet_id: "",
  amount: 0,
  transaction_type: "Expense",
  category: "Food",
  description: "",
  transaction_date: undefined,
  destination_wallet_id: undefined,
});

const transactionTypes = ["Income", "Expense", "Transfer"];

const incomeCategories = ["Salary", "Freelance", "Investment", "Gift", "OtherIncome", "Custom"];
const expenseCategories = [
  "Food",
  "Transportation",
  "Housing",
  "Utilities",
  "Healthcare",
  "Entertainment",
  "Shopping",
  "Education",
  "Travel",
  "Insurance",
  "OtherExpense",
  "Custom",
];

const customCategory = ref("");
const isCustomCategory = computed(() => {
  return newTransaction.value.category === "Custom";
});

const categoryOptions = computed(() => {
  if (newTransaction.value.transaction_type === "Income") {
    return incomeCategories;
  } else if (newTransaction.value.transaction_type === "Expense") {
    return expenseCategories;
  } else {
    return [];
  }
});

const walletOptions = computed(() => {
  return walletStore.wallets.map((w) => ({ label: w.name, value: w.id }));
});

const destinationWalletOptions = computed(() => {
  return walletStore.wallets
    .filter((w) => w.id !== newTransaction.value.wallet_id)
    .map((w) => ({ label: w.name, value: w.id }));
});

const showDestinationWallet = computed(() => {
  return newTransaction.value.transaction_type === "Transfer";
});

watch(
  () => newTransaction.value.transaction_type,
  (newType) => {
    if (newType === "Income") {
      newTransaction.value.category = incomeCategories[0] || "Salary";
    } else if (newType === "Expense") {
      newTransaction.value.category = expenseCategories[0] || "Food";
    } else if (newType === "Transfer") {
      newTransaction.value.category = "";
    }
    customCategory.value = "";
    
    if (newType !== "Transfer") {
      newTransaction.value.destination_wallet_id = undefined;
    }
  },
);

const validateAmountInput = (event: Event) => {
  const input = event.target as HTMLInputElement;
  let value = input.value;

  value = value.replace(/[^0-9.]/g, "");

  const parts = value.split(".");
  if (parts.length > 2) {
    value = `${parts[0]}.${parts.slice(1).join("")}`;
  }

  if (parts.length === 2 && parts[1] && parts[1].length > 2) {
    value = `${parts[0]}.${parts[1].substring(0, 2)}`;
  }

  input.value = value;
  newTransactionAmount.value = value;
};

const resetForm = () => {
  newTransaction.value = {
    wallet_id: walletStore.wallets.length > 0 ? walletStore.wallets[0]?.id || "" : "",
    amount: 0,
    transaction_type: "Expense",
    category: "Food",
    description: "",
    transaction_date: undefined,
    destination_wallet_id: undefined,
  };
  newTransactionAmount.value = "0";
  customCategory.value = "";
  transactionDate.value = new Date();
};

const handleCreate = () => {
  if (!newTransaction.value.wallet_id) {
    useToastStore().showError(t("transactions.wallet") + " is required");
    return;
  }

  const amount = newTransactionAmount.value ? parseFloat(newTransactionAmount.value) : 0;
  if (amount <= 0) {
    useToastStore().showError(t("transactions.amount_required"));
    return;
  }

  if (newTransaction.value.transaction_type === "Transfer" && !newTransaction.value.destination_wallet_id) {
    useToastStore().showError(t("transactions.destination_wallet") + " is required");
    return;
  }

  // Use custom category if "Custom" is selected and custom category is entered
  const finalCategory = isCustomCategory.value && customCategory.value 
    ? customCategory.value 
    : newTransaction.value.category;

  if (!finalCategory || (isCustomCategory.value && !customCategory.value)) {
    useToastStore().showError(t("transactions.category") + " is required");
    return;
  }

  newTransaction.value.amount = amount;
  newTransaction.value.category = finalCategory;
  newTransaction.value.transaction_date = transactionDate.value.toISOString();
  emit("create", { ...newTransaction.value });
};

const handleVisibilityChange = (value: boolean) => {
  if (value) {
    resetForm();
  }
  emit("update:visible", value);
};
</script>

<template>
  <GlassDialog :visible="visible" @update:visible="handleVisibilityChange" :header="t('transactions.create_transaction')">
    <div class="space-y-4">
      <div>
        <label class="block mb-2 text-white/90"><i class="pi pi-wallet mr-2"></i>{{ t("transactions.wallet") }}</label>
        <GlassDropdown v-model="newTransaction.wallet_id" :options="walletOptions" optionLabel="label" optionValue="value" />
      </div>
      <div>
        <label class="block mb-2 text-white/90"><i class="pi pi-dollar mr-2"></i>{{ t("transactions.amount") }}</label>
        <GlassInput v-model="newTransactionAmount" @input="validateAmountInput" />
      </div>
      <div>
        <label class="block mb-2 text-white/90"><i class="pi pi-tag mr-2"></i>{{ t("transactions.type") }}</label>
        <GlassDropdown v-model="newTransaction.transaction_type" :options="transactionTypes" />
      </div>
      <div v-if="newTransaction.transaction_type !== 'Transfer'">
        <label class="block mb-2 text-white/90"><i class="pi pi-bookmark mr-2"></i>{{ t("transactions.category") }}</label>
        <GlassDropdown v-model="newTransaction.category" :options="categoryOptions" />
      </div>
      <div v-if="isCustomCategory">
        <label class="block mb-2 text-white/90"><i class="pi pi-pencil mr-2"></i>{{ t("transactions.custom_category") }}</label>
        <GlassInput v-model="customCategory" :placeholder="t('transactions.enter_custom_category')" />
      </div>
      <div v-if="showDestinationWallet">
        <label class="block mb-2 text-white/90"><i class="pi pi-arrow-right mr-2"></i>{{ t("transactions.destination_wallet") }}</label>
        <GlassDropdown v-model="newTransaction.destination_wallet_id" :options="destinationWalletOptions" optionLabel="label" optionValue="value" />
      </div>
      <div>
        <label class="block mb-2 text-white/90"><i class="pi pi-calendar mr-2"></i>{{ t("transactions.date") }}</label>
        <Calendar v-model="transactionDate" showTime hourFormat="24" dateFormat="yy-mm-dd"
          :pt="{
            input: {
              class: 'w-full backdrop-blur-xl! bg-white/10! border! border-white/30! text-white! p-3 rounded-lg focus:outline-none focus:ring-2 focus:ring-white/50',
            },
            panel: {
              class: 'backdrop-blur-2xl! bg-white/10! border! border-white/30! shadow-xl!',
            },
            header: {
              class: 'bg-transparent! text-white!',
            },
            content: {
              class: 'bg-transparent! text-white!',
            },
          }" />
      </div>
      <div>
        <label class="block mb-2 text-white/90"><i class="pi pi-comment mr-2"></i>{{ t("transactions.description") }}</label>
        <Textarea v-model="newTransaction.description" rows="3" :placeholder="t('transactions.enter_description')"
          :pt="{
            root: {
              class: 'w-full backdrop-blur-xl! bg-white/10! border! border-white/30! text-white! p-3 rounded-lg focus:outline-none focus:ring-2 focus:ring-white/50',
            },
          }" />
      </div>
    </div>
    <template #footer>
      <GlassButton :label="t('settings.save_settings')" icon="pi pi-check" @click="handleCreate" :loading="loading" />
    </template>
  </GlassDialog>
</template>
