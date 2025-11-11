<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";

import type { CreateTransaction, Transaction, UpdateTransaction } from "@/services/transaction/types";

import GlassButton from "@/components/glass/GlassButton.vue";
import GlassCard from "@/components/glass/GlassCard.vue";
import TransactionCard from "@/components/transactions/TransactionCard.vue";
import TransactionCreateDialog from "@/components/transactions/TransactionCreateDialog.vue";
import TransactionDeleteDialog from "@/components/transactions/TransactionDeleteDialog.vue";
import TransactionEditDialog from "@/components/transactions/TransactionEditDialog.vue";
import { useTransactionStore } from "@/stores/transaction";
import { useWalletStore } from "@/stores/wallet";

const { t } = useI18n();
const transactionStore = useTransactionStore();
const walletStore = useWalletStore();

const showCreateDialog = ref(false);
const showEditDialog = ref(false);
const showDeleteDialog = ref(false);
const selectedTransaction = ref<Transaction | null>(null);

// Group transactions by date
const transactionsByDate = computed(() => {
  const grouped = new Map<string, Transaction[]>();

  transactionStore.transactions.forEach((transaction) => {
    const date = new Date(transaction.transaction_date).toDateString();
    if (!grouped.has(date)) {
      grouped.set(date, []);
    }
    grouped.get(date)!.push(transaction);
  });

  // Sort by date descending (most recent first)
  const sortedEntries = Array.from(grouped.entries()).sort(
    ([a], [b]) => new Date(b).getTime() - new Date(a).getTime(),
  );

  return sortedEntries.map(([date, transactions]) => ({
    date,
    formattedDate: formatDateGroup(date),
    transactions: transactions.sort(
      (a, b) =>
        new Date(b.transaction_date).getTime() -
        new Date(a.transaction_date).getTime(),
    ),
  }));
});

const formatDateGroup = (dateString: string) => {
  const date = new Date(dateString);
  const today = new Date();
  const yesterday = new Date(today);
  yesterday.setDate(yesterday.getDate() - 1);

  if (date.toDateString() === today.toDateString()) {
    return "Today";
  } else if (date.toDateString() === yesterday.toDateString()) {
    return "Yesterday";
  } else {
    return new Intl.DateTimeFormat("en-US", {
      weekday: "long",
      year: "numeric",
      month: "long",
      day: "numeric",
    }).format(date);
  }
};

onMounted(async () => {
  await Promise.all([
    walletStore.loadWallets(),
    transactionStore.loadTransactions(),
  ]);
});

const openCreateDialog = () => {
  showCreateDialog.value = true;
};

const createTransaction = async (transaction: CreateTransaction) => {
  const success = await transactionStore.createTransaction(transaction);
  if (success) {
    showCreateDialog.value = false;
  }
};

const openEditDialog = (transaction: Transaction) => {
  selectedTransaction.value = transaction;
  showEditDialog.value = true;
};

const updateTransaction = async (id: string, transaction: UpdateTransaction) => {
  const success = await transactionStore.updateTransaction(id, transaction);
  if (success) {
    showEditDialog.value = false;
    selectedTransaction.value = null;
  }
};

const openDeleteDialog = (transaction: Transaction) => {
  selectedTransaction.value = transaction;
  showDeleteDialog.value = true;
};

const deleteTransaction = async (id: string) => {
  const success = await transactionStore.deleteTransaction(id);
  if (success) {
    showDeleteDialog.value = false;
    selectedTransaction.value = null;
  }
};
</script>

<template>
  <div class="flex items-center justify-center min-h-screen p-4 bg-gradient-to-b from-blue-300 to-blue-500">
    <div class="w-full max-w-6xl">
      <GlassCard class="mb-6">
        <template #title>
          <div class="flex items-center justify-between text-white">
            <div class="flex items-center gap-3">
              <i class="pi pi-list text-2xl"></i>
              <span class="text-2xl font-medium">{{ t("transactions.title") }}</span>
            </div>
            <GlassButton :label="t('transactions.create_transaction')" icon="pi pi-plus" @click="openCreateDialog" />
          </div>
        </template>
        <template #content>
          <div v-if="transactionStore.loading" class="flex justify-center py-8 text-white">
            <ProgressSpinner style="width: 50px; height: 50px" strokeWidth="4" fill="transparent"
              animationDuration="1s" />
          </div>

          <div v-else-if="transactionStore.transactions.length === 0" class="text-center py-12 text-white">
            <i class="pi pi-list text-6xl mb-4 opacity-50"></i>
            <p class="text-xl mb-6 opacity-80">{{ t("transactions.no_transactions") }}</p>
            <GlassButton :label="t('transactions.create_transaction')" icon="pi pi-plus" @click="openCreateDialog" />
          </div>

          <div v-else class="space-y-6">
            <div v-for="group in transactionsByDate" :key="group.date" class="space-y-3">
              <h3 class="text-lg font-semibold text-white/90 px-2">{{ group.formattedDate }}</h3>
              <div class="space-y-2">
                <TransactionCard
                  v-for="transaction in group.transactions"
                  :key="transaction.id"
                  :transaction="transaction"
                  @edit="openEditDialog"
                  @delete="openDeleteDialog"
                />
              </div>
            </div>
          </div>
        </template>
      </GlassCard>
    </div>

    <!-- Dialogs -->
    <TransactionCreateDialog
      v-model:visible="showCreateDialog"
      :loading="transactionStore.loading"
      @create="createTransaction"
    />
    <TransactionEditDialog
      v-model:visible="showEditDialog"
      :loading="transactionStore.loading"
      :transaction="selectedTransaction"
      @update="updateTransaction"
    />
    <TransactionDeleteDialog
      v-model:visible="showDeleteDialog"
      :loading="transactionStore.loading"
      :transaction="selectedTransaction"
      @delete="deleteTransaction"
    />
  </div>
</template>
