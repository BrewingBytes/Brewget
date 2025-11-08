<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";

import type { Transaction } from "@/services/transaction/types";

import TransactionCard from "./TransactionCard.vue";

interface Props {
  transactions: Transaction[];
}

interface Emits {
  (event: "edit", transaction: Transaction): void;
  (event: "delete", transaction: Transaction): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();
const { t } = useI18n();

// Group transactions by date
const groupedTransactions = computed(() => {
  const groups = new Map<string, Transaction[]>();

  props.transactions.forEach((transaction) => {
    const date = new Date(transaction.transaction_date);
    const dateKey = date.toISOString().split("T")[0]; // YYYY-MM-DD

    if (!groups.has(dateKey)) {
      groups.set(dateKey, []);
    }
    groups.get(dateKey)!.push(transaction);
  });

  // Convert to array and sort by date (newest first)
  return Array.from(groups.entries())
    .sort(([a], [b]) => new Date(b).getTime() - new Date(a).getTime())
    .map(([dateKey, transactions]) => ({
      date: dateKey,
      dateDisplay: formatDateGroup(dateKey),
      transactions: transactions.sort(
        (a, b) =>
          new Date(b.transaction_date).getTime() - new Date(a.transaction_date).getTime(),
      ),
    }));
});

const formatDateGroup = (dateString: string) => {
  const date = new Date(dateString);
  const today = new Date();
  const yesterday = new Date(today);
  yesterday.setDate(yesterday.getDate() - 1);

  // Reset time parts for accurate comparison
  today.setHours(0, 0, 0, 0);
  yesterday.setHours(0, 0, 0, 0);
  date.setHours(0, 0, 0, 0);

  if (date.getTime() === today.getTime()) {
    return t("transactions.today");
  } else if (date.getTime() === yesterday.getTime()) {
    return t("transactions.yesterday");
  } else {
    return new Intl.DateTimeFormat("en-US", {
      weekday: "long",
      year: "numeric",
      month: "long",
      day: "numeric",
    }).format(date);
  }
};
</script>

<template>
  <div class="space-y-6">
    <div
      v-if="groupedTransactions.length === 0"
      class="text-center py-12 text-white"
    >
      <i class="pi pi-inbox text-6xl mb-4 opacity-50"></i>
      <p class="text-xl opacity-80">{{ t("transactions.no_transactions") }}</p>
    </div>

    <div
      v-for="group in groupedTransactions"
      :key="group.date"
      class="space-y-3"
    >
      <h3 class="text-lg font-medium text-white/90 px-2">
        {{ group.dateDisplay }}
      </h3>
      <div class="space-y-2">
        <TransactionCard
          v-for="transaction in group.transactions"
          :key="transaction.id"
          :transaction="transaction"
          @edit="emit('edit', transaction)"
          @delete="emit('delete', transaction)"
        />
      </div>
    </div>
  </div>
</template>
