<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";

import type { CreateWallet, UpdateWallet, Wallet } from "@/services/transaction/types";

import GlassButton from "@/components/glass/GlassButton.vue";
import GlassCard from "@/components/glass/GlassCard.vue";
import WalletCreateDialog from "@/components/wallets/WalletCreateDialog.vue";
import WalletDeleteDialog from "@/components/wallets/WalletDeleteDialog.vue";
import WalletEditDialog from "@/components/wallets/WalletEditDialog.vue";
import WalletTypeGroup from "@/components/wallets/WalletTypeGroup.vue";
import { useWalletStore } from "@/stores/wallet";

const { t } = useI18n();
const walletStore = useWalletStore();

const showCreateDialog = ref(false);
const showEditDialog = ref(false);
const showDeleteDialog = ref(false);
const selectedWallet = ref<Wallet | null>(null);

// Helper to get wallet type display name
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

// Group wallets by wallet type
const walletsByType = computed(() => {
  const grouped = new Map<string, Wallet[]>();

  walletStore.wallets.forEach((wallet) => {
    const walletType = wallet.wallet_type || "Account";
    if (!grouped.has(walletType)) {
      grouped.set(walletType, []);
    }
    grouped.get(walletType)!.push(wallet);
  });

  // Sort by wallet type order
  const typeOrder = ["Account", "Savings", "Deposit", "CreditCard", "Loan"];
  const sortedEntries = Array.from(grouped.entries()).sort(
    ([a], [b]) => typeOrder.indexOf(a) - typeOrder.indexOf(b),
  );

  return sortedEntries.map(([walletType, wallets]) => ({
    walletType,
    label: getWalletTypeLabel(walletType),
    wallets,
  }));
});

onMounted(async () => {
  await walletStore.loadWallets();
});

const openCreateDialog = () => {
  showCreateDialog.value = true;
};

const createWallet = async (wallet: CreateWallet) => {
  const success = await walletStore.createWallet(wallet);
  if (success) {
    showCreateDialog.value = false;
  }
};

const openEditDialog = (wallet: Wallet) => {
  selectedWallet.value = wallet;
  showEditDialog.value = true;
};

const updateWallet = async (id: string, wallet: UpdateWallet) => {
  const success = await walletStore.updateWallet(id, wallet);
  if (success) {
    showEditDialog.value = false;
    selectedWallet.value = null;
  }
};

const openDeleteDialog = (wallet: Wallet) => {
  selectedWallet.value = wallet;
  showDeleteDialog.value = true;
};

const deleteWallet = async (id: string) => {
  const success = await walletStore.deleteWallet(id);
  if (success) {
    showDeleteDialog.value = false;
    selectedWallet.value = null;
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
              <i class="pi pi-wallet text-2xl"></i>
              <span class="text-2xl font-medium">{{ t("wallets.title") }}</span>
            </div>
            <GlassButton :label="t('wallets.create_wallet')" icon="pi pi-plus" @click="openCreateDialog" />
          </div>
        </template>
        <template #content>
          <div v-if="walletStore.loading" class="flex justify-center py-8 text-white">
            <ProgressSpinner style="width: 50px; height: 50px" strokeWidth="4" fill="transparent"
              animationDuration="1s" />
          </div>

          <div v-else-if="walletStore.wallets.length === 0" class="text-center py-12 text-white">
            <i class="pi pi-wallet text-6xl mb-4 opacity-50"></i>
            <p class="text-xl mb-6 opacity-80">{{ t("wallets.no_wallets") }}</p>
            <GlassButton :label="t('wallets.create_wallet')" icon="pi pi-plus" @click="openCreateDialog" />
          </div>

          <div v-else class="space-y-6">
            <WalletTypeGroup v-for="group in walletsByType" :key="group.walletType" :walletType="group.walletType"
              :label="group.label" :wallets="group.wallets" @edit="openEditDialog" @delete="openDeleteDialog" />
          </div>
        </template>
      </GlassCard>
    </div>

    <!-- Dialogs -->
    <WalletCreateDialog v-model:visible="showCreateDialog" :loading="walletStore.loading" @create="createWallet" />
    <WalletEditDialog v-model:visible="showEditDialog" :loading="walletStore.loading" :wallet="selectedWallet"
      @update="updateWallet" />
    <WalletDeleteDialog v-model:visible="showDeleteDialog" :loading="walletStore.loading" :wallet="selectedWallet"
      @delete="deleteWallet" />
  </div>
</template>
