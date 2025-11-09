<script setup lang="ts">
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";

import type { Transaction, UpdateTransaction } from "@/services/transaction/types";

import GlassButton from "@/components/glass/GlassButton.vue";
import GlassDialog from "@/components/glass/GlassDialog.vue";
import GlassDropdown from "@/components/glass/GlassDropdown.vue";
import GlassInput from "@/components/glass/GlassInput.vue";
import { useToastStore } from "@/stores/toast";

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

const editTransactionAmount = ref("0");
const editTransactionDate = ref<Date>(new Date());
const editTransaction = ref<UpdateTransaction>({
  amount: 0,
  category: "",
  description: "",
  transaction_date: undefined,
});

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
const isCustomCategory = ref(false);

const getCategoryOptions = (transactionType: string) => {
  if (transactionType === "Income") {
    return incomeCategories;
  } else if (transactionType === "Expense") {
    return expenseCategories;
  } else {
    return [];
  }
};

watch(
  () => props.transaction,
  (newTransaction) => {
    if (newTransaction) {
      // Check if the category is one of the predefined ones
      const categoryOptions = getCategoryOptions(newTransaction.transaction_type);
      const isPredefined = categoryOptions.includes(newTransaction.category);
      
      if (isPredefined) {
        isCustomCategory.value = false;
        editTransaction.value = {
          amount: newTransaction.amount,
          category: newTransaction.category,
          description: newTransaction.description || "",
          transaction_date: undefined,
        };
      } else {
        // It's a custom category
        isCustomCategory.value = true;
        customCategory.value = newTransaction.category;
        editTransaction.value = {
          amount: newTransaction.amount,
          category: "Custom",
          description: newTransaction.description || "",
          transaction_date: undefined,
        };
      }
      editTransactionAmount.value = newTransaction.amount.toString();
      editTransactionDate.value = new Date(newTransaction.transaction_date);
    }
  },
  { immediate: true },
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
  editTransactionAmount.value = value;
};

const handleUpdate = () => {
  if (!props.transaction) {return;}

  const amount = editTransactionAmount.value ? parseFloat(editTransactionAmount.value) : 0;
  if (amount <= 0) {
    useToastStore().showError(t("transactions.amount_required"));
    return;
  }

  // Use custom category if "Custom" is selected and custom category is entered
  const finalCategory = isCustomCategory.value && customCategory.value 
    ? customCategory.value 
    : editTransaction.value.category;

  if (!finalCategory || (isCustomCategory.value && !customCategory.value)) {
    useToastStore().showError(`${t("transactions.category")  } is required`);
    return;
  }

  editTransaction.value.amount = amount;
  editTransaction.value.category = finalCategory;
  // Format date as "YYYY-MM-DD HH:MM:SS" for backend NaiveDateTime
  const year = editTransactionDate.value.getFullYear();
  const month = String(editTransactionDate.value.getMonth() + 1).padStart(2, '0');
  const day = String(editTransactionDate.value.getDate()).padStart(2, '0');
  const hours = String(editTransactionDate.value.getHours()).padStart(2, '0');
  const minutes = String(editTransactionDate.value.getMinutes()).padStart(2, '0');
  const seconds = String(editTransactionDate.value.getSeconds()).padStart(2, '0');
  editTransaction.value.transaction_date = `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
  emit("update", props.transaction.id, { ...editTransaction.value });
};
</script>

<template>
  <GlassDialog :visible="visible" @update:visible="emit('update:visible', $event)" :header="t('transactions.edit_transaction')">
    <div v-if="transaction" class="space-y-4">
      <div>
        <label class="block mb-2 text-white/90"><i class="pi pi-dollar mr-2"></i>{{ t("transactions.amount") }}</label>
        <GlassInput v-model="editTransactionAmount" @input="validateAmountInput" />
      </div>
      <div v-if="transaction.transaction_type !== 'Transfer'">
        <label class="block mb-2 text-white/90"><i class="pi pi-bookmark mr-2"></i>{{ t("transactions.category") }}</label>
        <GlassDropdown v-model="editTransaction.category" :options="getCategoryOptions(transaction.transaction_type)" @change="isCustomCategory = editTransaction.category === 'Custom'" />
      </div>
      <div v-if="isCustomCategory">
        <label class="block mb-2 text-white/90"><i class="pi pi-pencil mr-2"></i>{{ t("transactions.custom_category") }}</label>
        <GlassInput v-model="customCategory" :placeholder="t('transactions.enter_custom_category')" />
      </div>
      <div>
        <label class="block mb-2 text-white/90"><i class="pi pi-calendar mr-2"></i>{{ t("transactions.date") }}</label>
        <Calendar v-model="editTransactionDate" showTime hourFormat="24" dateFormat="yy-mm-dd"
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
        <Textarea v-model="editTransaction.description" rows="3" :placeholder="t('transactions.enter_description')"
          :pt="{
            root: {
              class: 'w-full backdrop-blur-xl! bg-white/10! border! border-white/30! text-white! p-3 rounded-lg focus:outline-none focus:ring-2 focus:ring-white/50',
            },
          }" />
      </div>
    </div>
    <template #footer>
      <GlassButton :label="t('settings.save_settings')" icon="pi pi-check" @click="handleUpdate" :loading="loading" />
    </template>
  </GlassDialog>
</template>
