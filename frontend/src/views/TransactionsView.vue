<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { useRoute } from "vue-router";

import type {
  CreateTransaction,
  Transaction,
  UpdateTransaction,
} from "@/services/transaction/types";

import CategoryManageDialog from "@/components/categories/CategoryManageDialog.vue";
import GlassButton from "@/components/glass/GlassButton.vue";
import GlassCard from "@/components/glass/GlassCard.vue";
import TransactionCreateDialog from "@/components/transactions/TransactionCreateDialog.vue";
import TransactionDeleteDialog from "@/components/transactions/TransactionDeleteDialog.vue";
import TransactionEditDialog from "@/components/transactions/TransactionEditDialog.vue";
import TransactionList from "@/components/transactions/TransactionList.vue";
import { useTransactionStore } from "@/stores/transaction";
import { useWalletStore } from "@/stores/wallet";

const { t } = useI18n();
const route = useRoute();
const transactionStore = useTransactionStore();
const walletStore = useWalletStore();

const showCreateDialog = ref(false);
const showEditDialog = ref(false);
const showDeleteDialog = ref(false);
const showCategoryDialog = ref(false);
const selectedTransaction = ref<Transaction | null>(null);

// Get wallet ID from route query if present
const walletIdFilter = computed(() => route.query.wallet as string | undefined);

// Get wallet name for filtered view
const filteredWalletName = computed(() => {
  if (!walletIdFilter.value) {
    return null;
  }
  const wallet = walletStore.wallets.find((w) => w.id === walletIdFilter.value);
  return wallet?.name || null;
});

// Filter transactions by wallet if wallet query param is present
const filteredTransactions = computed(() => {
  if (!walletIdFilter.value) {
    return transactionStore.transactions;
  }
  return transactionStore.transactions.filter(
    (t) => t.wallet_id === walletIdFilter.value,
  );
});

onMounted(async () => {
  // Load wallets first if not already loaded
  if (walletStore.wallets.length === 0) {
    await walletStore.loadWallets();
  }

  // Load all transactions or wallet-specific transactions
  if (walletIdFilter.value) {
    await transactionStore.loadWalletTransactions(walletIdFilter.value);
  } else {
    await transactionStore.loadTransactions();
  }
});

// Reload transactions when wallet filter changes
watch(walletIdFilter, async (newWalletId) => {
  if (newWalletId) {
    await transactionStore.loadWalletTransactions(newWalletId);
  } else {
    await transactionStore.loadTransactions();
  }
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

const openCategoryDialog = () => {
  showCategoryDialog.value = true;
};
</script>

<template>
  <div
    class="flex items-center justify-center min-h-screen p-4 bg-gradient-to-b from-blue-300 to-blue-500"
  >
    <div class="w-full max-w-6xl">
      <GlassCard class="mb-6">
        <template #title>
          <div class="flex items-center justify-between text-white">
            <div class="flex items-center gap-3">
              <i class="pi pi-receipt text-2xl"></i>
              <div>
                <span class="text-2xl font-medium">{{ t("transactions.title") }}</span>
                <p v-if="filteredWalletName" class="text-sm font-normal opacity-80">
                  {{ filteredWalletName }}
                </p>
              </div>
            </div>
            <div class="flex gap-2">
              <GlassButton
                :label="t('categories.manage_categories')"
                icon="pi pi-folder"
                @click="openCategoryDialog"
              />
              <GlassButton
                :label="t('transactions.create_transaction')"
                icon="pi pi-plus"
                @click="openCreateDialog"
              />
            </div>
          </div>
        </template>
        <template #content>
          <div
            v-if="transactionStore.loading"
            class="flex justify-center py-8 text-white"
          >
            <ProgressSpinner
              style="width: 50px; height: 50px"
              strokeWidth="4"
              fill="transparent"
              animationDuration="1s"
            />
          </div>

          <div
            v-else-if="filteredTransactions.length === 0"
            class="text-center py-12 text-white"
          >
            <i class="pi pi-inbox text-6xl mb-4 opacity-50"></i>
            <p class="text-xl mb-6 opacity-80">{{ t("transactions.no_transactions") }}</p>
            <GlassButton
              :label="t('transactions.create_transaction')"
              icon="pi pi-plus"
              @click="openCreateDialog"
            />
          </div>

          <TransactionList
            v-else
            :transactions="filteredTransactions"
            @edit="openEditDialog"
            @delete="openDeleteDialog"
          />
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
    <CategoryManageDialog v-model:visible="showCategoryDialog" />
  </div>
</template>
