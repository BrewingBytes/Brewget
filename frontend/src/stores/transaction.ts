import { defineStore } from "pinia";
import { ref } from "vue";

import { useToastStore } from "./toast";

import type {
  CreateTransaction,
  Transaction,
  UpdateTransaction,
} from "@/services/transaction/types";

import i18n from "@/i18n";
import { transactionService } from "@/services/transaction";
import { ServerStatus } from "@/services/types";

export const useTransactionStore = defineStore("transaction", () => {
  const transactions = ref<Transaction[]>([]);
  const loading = ref(false);

  async function loadTransactions(): Promise<void> {
    loading.value = true;
    try {
      const response = await transactionService.getTransactions();

      if (response.status !== ServerStatus.NO_ERROR) {
        useToastStore().showError(i18n.global.t("transactions.failed_to_load"));
        return;
      }

      transactions.value = response.data;
    } finally {
      loading.value = false;
    }
  }

  async function loadWalletTransactions(walletId: string): Promise<void> {
    loading.value = true;
    try {
      const response = await transactionService.getWalletTransactions(walletId);

      if (response.status !== ServerStatus.NO_ERROR) {
        useToastStore().showError(i18n.global.t("transactions.failed_to_load"));
        return;
      }

      transactions.value = response.data;
    } finally {
      loading.value = false;
    }
  }

  async function createTransaction(transaction: CreateTransaction): Promise<boolean> {
    loading.value = true;
    try {
      const response = await transactionService.createTransaction(transaction);

      if (response.status === ServerStatus.NO_ERROR || response.status === ServerStatus.CREATED) {
        transactions.value.unshift(response.data as Transaction);
        useToastStore().showInfo(i18n.global.t("transactions.transaction_created"));
        return true;
      }

      useToastStore().showError(i18n.global.t("transactions.failed_to_create"));
      return false;
    } finally {
      loading.value = false;
    }
  }

  async function updateTransaction(
    id: string,
    transaction: UpdateTransaction,
  ): Promise<boolean> {
    loading.value = true;
    try {
      const response = await transactionService.updateTransaction(id, transaction);

      if (response.status !== ServerStatus.NO_ERROR) {
        useToastStore().showError(i18n.global.t("transactions.failed_to_update"));
        return false;
      }

      const index = transactions.value.findIndex((t) => t.id === id);
      if (index !== -1) {
        transactions.value[index] = response.data;
      }
      useToastStore().showInfo(i18n.global.t("transactions.transaction_updated"));
      return true;
    } finally {
      loading.value = false;
    }
  }

  async function deleteTransaction(id: string): Promise<boolean> {
    loading.value = true;
    try {
      const response = await transactionService.deleteTransaction(id);

      if (response.status !== ServerStatus.NO_ERROR && response.status !== ServerStatus.NO_CONTENT) {
        useToastStore().showError(i18n.global.t("transactions.failed_to_delete"));
        return false;
      }

      transactions.value = transactions.value.filter((t) => t.id !== id);
      useToastStore().showInfo(i18n.global.t("transactions.transaction_deleted"));
      return true;
    } finally {
      loading.value = false;
    }
  }

  return {
    transactions,
    loading,
    loadTransactions,
    loadWalletTransactions,
    createTransaction,
    updateTransaction,
    deleteTransaction,
  };
});
