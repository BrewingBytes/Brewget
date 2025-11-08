<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";

import type { CreateTransaction } from "@/services/transaction/types";

import GlassButton from "@/components/glass/GlassButton.vue";
import GlassDialog from "@/components/glass/GlassDialog.vue";
import GlassDropdown from "@/components/glass/GlassDropdown.vue";
import GlassInput from "@/components/glass/GlassInput.vue";
import { useCustomCategoryStore } from "@/stores/customCategory";
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
const customCategoryStore = useCustomCategoryStore();

const newTransactionAmount = ref("0");
const newTransactionDate = ref(new Date());
const newTransaction = ref<CreateTransaction>({
  wallet_id: "",
  amount: 0,
  transaction_type: "Expense",
  category: "",
  description: "",
  transaction_date: new Date().toISOString(),
});

const transactionTypes = ["Income", "Expense", "Transfer"];

// Built-in categories by type
const builtInCategories = {
  Income: ["Salary", "Freelance", "Investment", "Gift", "Other"],
  Expense: [
    "Food",
    "Transport",
    "Shopping",
    "Entertainment",
    "Bills",
    "Healthcare",
    "Education",
    "Other",
  ],
  Transfer: ["Transfer"],
};

// Combine built-in and custom categories based on selected transaction type
const availableCategories = computed(() => {
  const type = newTransaction.value.transaction_type as keyof typeof builtInCategories;
  const builtin = builtInCategories[type] || [];

  // Get custom categories for this transaction type
  const custom = customCategoryStore.categories
    .filter((c) => c.transaction_type === type)
    .map((c) => c.name);

  return [...builtin, ...custom];
});

// Wallet options for dropdown
const walletOptions = computed(() =>
  walletStore.wallets.map((w) => ({
    label: `${w.name} (${w.currency})`,
    value: w.id,
  })),
);

const validateAmountInput = (event: Event) => {
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
  newTransactionAmount.value = value;
};

const resetForm = () => {
  newTransaction.value = {
    wallet_id: walletStore.wallets.length > 0 ? walletStore.wallets[0].id : "",
    amount: 0,
    transaction_type: "Expense",
    category: "",
    description: "",
    transaction_date: new Date().toISOString(),
  };
  newTransactionAmount.value = "0";
  newTransactionDate.value = new Date();
};

const handleCreate = () => {
  // Validate required fields
  if (!newTransaction.value.wallet_id) {
    useToastStore().showError(t("transactions.wallet_required"));
    return;
  }

  if (!newTransaction.value.category) {
    useToastStore().showError(t("transactions.category_required"));
    return;
  }

  // Parse amount from string
  const amount = newTransactionAmount.value ? parseFloat(newTransactionAmount.value) : 0;

  if (amount <= 0) {
    useToastStore().showError(t("transactions.amount_required"));
    return;
  }

  newTransaction.value.amount = amount;

  // Convert Date object to ISO string for API
  newTransaction.value.transaction_date = newTransactionDate.value.toISOString();

  emit("create", newTransaction.value);
};

// Reset form when dialog opens
const handleVisibilityChange = (value: boolean) => {
  if (value) {
    resetForm();
  }
  emit("update:visible", value);
};

// Load wallets and categories when component mounts
onMounted(async () => {
  if (walletStore.wallets.length === 0) {
    await walletStore.loadWallets();
  }
  if (customCategoryStore.categories.length === 0) {
    await customCategoryStore.loadCategories();
  }
});

defineExpose({ resetForm });
</script>

<template>
  <GlassDialog
    :visible="visible"
    @update:visible="handleVisibilityChange"
    :header="t('transactions.create_transaction')"
    class="w-full max-w-2xl"
  >
    <div class="space-y-4">
      <div>
        <label class="block mb-2 text-white/90">
          <i class="pi pi-wallet mr-2"></i>{{ t("transactions.wallet") }}
        </label>
        <Dropdown
          v-model="newTransaction.wallet_id"
          :options="walletOptions"
          optionLabel="label"
          optionValue="value"
          :placeholder="t('transactions.select_wallet')"
          class="w-full bg-transparent! border-white! text-white!"
        />
      </div>

      <div>
        <label class="block mb-2 text-white/90">
          <i class="pi pi-tag mr-2"></i>{{ t("transactions.type") }}
        </label>
        <GlassDropdown
          v-model="newTransaction.transaction_type"
          :options="transactionTypes"
        />
      </div>

      <div>
        <label class="block mb-2 text-white/90">
          <i class="pi pi-dollar mr-2"></i>{{ t("transactions.amount") }}
        </label>
        <GlassInput
          v-model="newTransactionAmount"
          @input="validateAmountInput"
          :placeholder="t('transactions.enter_amount')"
        />
      </div>

      <div>
        <label class="block mb-2 text-white/90">
          <i class="pi pi-folder mr-2"></i>{{ t("transactions.category") }}
        </label>
        <GlassDropdown
          v-model="newTransaction.category"
          :options="availableCategories"
        />
      </div>

      <div>
        <label class="block mb-2 text-white/90">
          <i class="pi pi-align-left mr-2"></i>{{ t("transactions.description") }}
        </label>
        <GlassInput
          v-model="newTransaction.description"
          :placeholder="t('transactions.enter_description')"
        />
      </div>

      <div>
        <label class="block mb-2 text-white/90">
          <i class="pi pi-calendar mr-2"></i>{{ t("transactions.date") }}
        </label>
        <Calendar
          v-model="newTransactionDate"
          showTime
          hourFormat="24"
          dateFormat="yy-mm-dd"
          class="w-full bg-transparent! border-white! text-white!"
          :pt="{
            root: {
              class: 'bg-transparent! border-white! text-white!',
            },
            input: {
              class: 'bg-transparent! border-white! text-white!',
            },
          }"
        />
      </div>
    </div>
    <template #footer>
      <GlassButton
        :label="t('settings.save_settings')"
        icon="pi pi-check"
        @click="handleCreate"
        :loading="loading"
      />
    </template>
  </GlassDialog>
</template>
