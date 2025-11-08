<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";

import type { Transaction, UpdateTransaction } from "@/services/transaction/types";

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
  transaction: Transaction | null;
}

interface Emits {
  (event: "update:visible", value: boolean): void;
  (event: "update", id: string, transaction: UpdateTransaction): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();
const { t } = useI18n();

const walletStore = useWalletStore();
const customCategoryStore = useCustomCategoryStore();

const editTransactionAmount = ref("0");
const editTransactionDate = ref(new Date());
const editTransaction = ref<UpdateTransaction>({
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
  const type = (editTransaction.value.transaction_type ||
    "Expense") as keyof typeof builtInCategories;
  const builtin = builtInCategories[type] || [];

  // Get custom categories for this transaction type
  const custom = customCategoryStore.categories
    .filter((c) => c.transaction_type === type)
    .map((c) => c.name);

  return [...builtin, ...custom];
});

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
  editTransactionAmount.value = value;
};

// Watch for transaction changes to update the form
watch(
  () => props.transaction,
  (newTransaction) => {
    if (newTransaction) {
      editTransaction.value = {
        amount: newTransaction.amount,
        transaction_type: newTransaction.transaction_type,
        category: newTransaction.category,
        description: newTransaction.description || "",
        transaction_date: newTransaction.transaction_date,
      };
      editTransactionAmount.value = newTransaction.amount.toString();
      editTransactionDate.value = new Date(newTransaction.transaction_date);
    }
  },
  { immediate: true },
);

const handleUpdate = () => {
  if (!props.transaction) {
    return;
  }

  // Validate required fields
  if (!editTransaction.value.category) {
    useToastStore().showError(t("transactions.category_required"));
    return;
  }

  // Parse amount from string
  const amount = editTransactionAmount.value ? parseFloat(editTransactionAmount.value) : 0;

  if (amount <= 0) {
    useToastStore().showError(t("transactions.amount_required"));
    return;
  }

  editTransaction.value.amount = amount;

  // Convert Date object to ISO string for API
  editTransaction.value.transaction_date = editTransactionDate.value.toISOString();

  emit("update", props.transaction.id, editTransaction.value);
};

// Load categories when component mounts
onMounted(async () => {
  if (customCategoryStore.categories.length === 0) {
    await customCategoryStore.loadCategories();
  }
});
</script>

<template>
  <GlassDialog
    :visible="visible"
    @update:visible="emit('update:visible', $event)"
    :header="t('transactions.edit_transaction')"
    class="w-full max-w-2xl"
  >
    <div class="space-y-4">
      <div>
        <label class="block mb-2 text-white/90">
          <i class="pi pi-tag mr-2"></i>{{ t("transactions.type") }}
        </label>
        <GlassDropdown
          v-model="editTransaction.transaction_type"
          :options="transactionTypes"
        />
      </div>

      <div>
        <label class="block mb-2 text-white/90">
          <i class="pi pi-dollar mr-2"></i>{{ t("transactions.amount") }}
        </label>
        <GlassInput
          v-model="editTransactionAmount"
          @input="validateAmountInput"
          :placeholder="t('transactions.enter_amount')"
        />
      </div>

      <div>
        <label class="block mb-2 text-white/90">
          <i class="pi pi-folder mr-2"></i>{{ t("transactions.category") }}
        </label>
        <GlassDropdown
          v-model="editTransaction.category"
          :options="availableCategories"
        />
      </div>

      <div>
        <label class="block mb-2 text-white/90">
          <i class="pi pi-align-left mr-2"></i>{{ t("transactions.description") }}
        </label>
        <GlassInput
          v-model="editTransaction.description"
          :placeholder="t('transactions.enter_description')"
        />
      </div>

      <div>
        <label class="block mb-2 text-white/90">
          <i class="pi pi-calendar mr-2"></i>{{ t("transactions.date") }}
        </label>
        <Calendar
          v-model="editTransactionDate"
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
        @click="handleUpdate"
        :loading="loading"
      />
    </template>
  </GlassDialog>
</template>
