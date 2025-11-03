import { defineStore } from "pinia";
import { ref } from "vue";

import { useToastStore } from "./toast";

import type { CreateWallet, UpdateWallet, Wallet } from "@/services/wallet";

import i18n from "@/i18n";
import { walletService } from "@/services/wallet";
import { ServerStatus } from "@/services/types";

export const useWalletStore = defineStore("wallet", () => {
  const wallets = ref<Wallet[]>([]);
  const loading = ref(false);

  async function loadWallets(): Promise<void> {
    loading.value = true;
    try {
      const response = await walletService.listWallets();

      if (response.status !== ServerStatus.NO_ERROR) {
        useToastStore().showError(i18n.global.t("wallets.failed_to_load"));
        return;
      }

      wallets.value = response.data;
    } finally {
      loading.value = false;
    }
  }

  async function createWallet(wallet: CreateWallet): Promise<boolean> {
    loading.value = true;
    try {
      const response = await walletService.createWallet(wallet);

      if (response.status !== ServerStatus.NO_ERROR) {
        useToastStore().showError(i18n.global.t("wallets.failed_to_create"));
        return false;
      }

      wallets.value.unshift(response.data);
      useToastStore().showInfo(i18n.global.t("wallets.wallet_created"));
      return true;
    } finally {
      loading.value = false;
    }
  }

  async function updateWallet(
    id: string,
    updates: UpdateWallet,
  ): Promise<boolean> {
    loading.value = true;
    try {
      const response = await walletService.updateWallet(id, updates);

      if (response.status !== ServerStatus.NO_ERROR) {
        useToastStore().showError(i18n.global.t("wallets.failed_to_update"));
        return false;
      }

      const index = wallets.value.findIndex((w) => w.id === id);
      if (index !== -1) {
        wallets.value[index] = response.data;
      }
      useToastStore().showInfo(i18n.global.t("wallets.wallet_updated"));
      return true;
    } finally {
      loading.value = false;
    }
  }

  async function deleteWallet(id: string): Promise<boolean> {
    loading.value = true;
    try {
      const response = await walletService.deleteWallet(id);

      if (response.status !== ServerStatus.NO_ERROR) {
        useToastStore().showError(i18n.global.t("wallets.failed_to_delete"));
        return false;
      }

      wallets.value = wallets.value.filter((w) => w.id !== id);
      useToastStore().showInfo(i18n.global.t("wallets.wallet_deleted"));
      return true;
    } finally {
      loading.value = false;
    }
  }

  return { wallets, loading, loadWallets, createWallet, updateWallet, deleteWallet };
});
