<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";

import type { Transaction } from "@/services/transaction/types";

interface Props {
  transaction: Transaction;
}

interface Emits {
  (event: "edit", transaction: Transaction): void;
  (event: "delete", transaction: Transaction): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();
const { t } = useI18n();

const formatCurrency = (amount: number) => {
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD",
    minimumFractionDigits: 2,
  }).format(amount);
};

const formatDate = (dateString: string) => {
  const date = new Date(dateString);
  return new Intl.DateTimeFormat("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  }).format(date);
};

const transactionTypeColor = computed(() => {
  switch (props.transaction.transaction_type) {
    case "Income":
      return "text-green-300";
    case "Expense":
      return "text-red-300";
    case "Transfer":
      return "text-blue-300";
    default:
      return "text-white";
  }
});

const transactionTypeIcon = computed(() => {
  switch (props.transaction.transaction_type) {
    case "Income":
      return "pi-arrow-down";
    case "Expense":
      return "pi-arrow-up";
    case "Transfer":
      return "pi-arrow-right-arrow-left";
    default:
      return "pi-circle";
  }
});

const amountPrefix = computed(() => {
  switch (props.transaction.transaction_type) {
    case "Income":
      return "+";
    case "Expense":
      return "-";
    default:
      return "";
  }
});
</script>

<template>
  <Card class="backdrop-blur-xl! bg-white/10! border! border-white/30! shadow-xl!">
    <template #content>
      <div class="flex justify-between items-start text-white">
        <div class="flex-1">
          <div class="flex items-center gap-2 mb-2">
            <i :class="`pi ${transactionTypeIcon} ${transactionTypeColor}`"></i>
            <span class="font-medium">{{
              t(`transactions.types.${transaction.transaction_type.toLowerCase()}`)
            }}</span>
            <span class="text-sm text-white/60">•</span>
            <span class="text-sm text-white/80">{{ transaction.category }}</span>
          </div>
          <div v-if="transaction.description" class="text-sm text-white/70 mb-2">
            {{ transaction.description }}
          </div>
          <div class="text-xs text-white/50">
            {{ formatDate(transaction.transaction_date) }}
          </div>
        </div>
        <div class="flex items-center gap-3">
          <div :class="`text-2xl font-bold ${transactionTypeColor}`">
            {{ amountPrefix }}{{ formatCurrency(transaction.amount) }}
          </div>
          <div class="flex gap-1">
            <Button
              icon="pi pi-pencil"
              text
              rounded
              @click="emit('edit', transaction)"
              class="text-white! hover:bg-white/20!"
              size="small"
            />
            <Button
              icon="pi pi-trash"
              text
              rounded
              severity="danger"
              @click="emit('delete', transaction)"
              class="text-red-300! hover:bg-red-500/20!"
              size="small"
            />
          </div>
        </div>
      </div>
    </template>
  </Card>
</template>

