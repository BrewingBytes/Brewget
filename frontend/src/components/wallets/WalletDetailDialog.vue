<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";

import type { Transaction, Wallet } from "@/services/transaction/types";

import GlassDialog from "@/components/glass/GlassDialog.vue";
import TransactionCard from "@/components/transactions/TransactionCard.vue";
import { useTransactionStore } from "@/stores/transaction";

interface Props {
  visible: boolean;
  wallet: Wallet | null;
}

interface Emits {
  (event: "update:visible", value: boolean): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();
const { t } = useI18n();

const transactionStore = useTransactionStore();
const loading = ref(false);

const formatCurrency = (amount: number, currency: string) => {
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: currency,
  }).format(amount);
};

// Group transactions by date
const transactionsByDate = computed(() => {
  const grouped = new Map<string, Transaction[]>();

  transactionStore.transactions.forEach((transaction) => {
    const date = new Date(transaction.transaction_date).toLocaleDateString();
    if (!grouped.has(date)) {
      grouped.set(date, []);
    }
    grouped.get(date)!.push(transaction);
  });

  // Sort by date descending
  const sortedEntries = Array.from(grouped.entries()).sort(
    ([a], [b]) => new Date(b).getTime() - new Date(a).getTime()
  );

  return sortedEntries.map(([date, transactions]) => ({
    date,
    transactions: transactions.sort(
      (a, b) =>
        new Date(b.transaction_date).getTime() -
        new Date(a.transaction_date).getTime()
    ),
  }));
});

// Load transactions when wallet changes
watch(
  () => props.wallet,
  async (newWallet) => {
    if (newWallet && props.visible) {
      loading.value = true;
      await transactionStore.loadWalletTransactions(newWallet.id);
      loading.value = false;
    }
  },
  { immediate: true }
);

watch(
  () => props.visible,
  async (visible) => {
    if (visible && props.wallet) {
      loading.value = true;
      await transactionStore.loadWalletTransactions(props.wallet.id);
      loading.value = false;
    }
  }
);

const getWalletTypeLabel = (walletType: string) => {
  const labels: Record<string, string> = {
    Account: t("wallets.types.account"),
    Savings: t("wallets.types.savings"),
    Deposit: t("wallets.types.deposit"),
    CreditCard: t("wallets.types.credit_card"),
    Loan: t("wallets.types.loan"),
  };
  return labels[walletType] || walletType;
};
</script>

<template>
  <GlassDialog
    :visible="visible"
    @update:visible="emit('update:visible', $event)"
    :header="wallet?.name || ''"
    class="w-full md:w-3/4 lg:w-2/3"
  >
    <div v-if="wallet" class="space-y-6">
      <!-- Wallet Overview -->
      <div class="backdrop-blur-xl bg-white/10 border border-white/30 rounded-lg p-6 shadow-xl">
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6 text-white">
          <div>
            <p class="text-sm text-white/70 mb-1">{{ t("wallets.balance") }}</p>
            <p class="text-3xl font-bold">
              {{ formatCurrency(wallet.balance, wallet.currency) }}
            </p>
          </div>
          <div>
            <p class="text-sm text-white/70 mb-1">{{ t("wallets.wallet_type") }}</p>
            <p class="text-xl">{{ getWalletTypeLabel(wallet.wallet_type) }}</p>
          </div>
          <div>
            <p class="text-sm text-white/70 mb-1">{{ t("wallets.currency") }}</p>
            <p class="text-xl">{{ wallet.currency }}</p>
          </div>
        </div>
      </div>

      <!-- Transactions List -->
      <div>
        <h3 class="text-xl font-semibold text-white mb-4 flex items-center gap-2">
          <i class="pi pi-list"></i>
          {{ t("transactions.title") }}
        </h3>

        <div v-if="loading" class="flex justify-center py-8 text-white">
          <ProgressSpinner style="width: 50px; height: 50px" strokeWidth="4" fill="transparent"
            animationDuration="1s" />
        </div>

        <div v-else-if="transactionsByDate.length === 0" class="text-center py-8 text-white/70">
          <i class="pi pi-inbox text-4xl mb-2"></i>
          <p>{{ t("transactions.no_transactions") }}</p>
        </div>

        <div v-else class="space-y-6">
          <div v-for="group in transactionsByDate" :key="group.date" class="space-y-3">
            <h4 class="text-sm font-semibold text-white/80 px-2">{{ group.date }}</h4>
            <div class="space-y-2">
              <TransactionCard
                v-for="transaction in group.transactions"
                :key="transaction.id"
                :transaction="transaction"
                @edit="() => {}"
                @delete="() => {}"
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  </GlassDialog>
</template>
