<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";

import type { CreateWallet, UpdateWallet } from "@/services/wallet";

import { useSettingsStore } from "@/stores/settings";
import { useWalletStore } from "@/stores/wallet";
import { glassButtonsStyles } from "@/utils/pts/glassButtons";

const walletStore = useWalletStore();
const settingsStore = useSettingsStore();
const { t } = useI18n();

// Dialog states
const showCreateDialog = ref(false);
const showEditDialog = ref(false);
const showDeleteDialog = ref(false);

// Form fields
const walletName = ref("");
const walletBalance = ref<number | null>(null);
const walletCurrency = ref("");
const walletType = ref("general");
const editingWalletId = ref("");
const deletingWalletId = ref("");

// Wallet types
const walletTypes = computed(() => [
  { label: t("wallets.types.general"), value: "general" },
  { label: t("wallets.types.checking"), value: "checking" },
  { label: t("wallets.types.savings"), value: "savings" },
  { label: t("wallets.types.cash"), value: "cash" },
  { label: t("wallets.types.credit"), value: "credit" },
]);

// Load wallets on mount
onMounted(async () => {
  await walletStore.loadWallets();
});

// Open create dialog
function openCreateDialog() {
  walletName.value = "";
  walletBalance.value = null;
  walletCurrency.value = settingsStore.settings?.currency || "USD";
  walletType.value = "general";
  showCreateDialog.value = true;
}

// Handle create wallet
async function handleCreate() {
  if (!walletName.value.trim()) {
    return;
  }

  const wallet: CreateWallet = {
    name: walletName.value,
    balance: walletBalance.value ?? undefined,
    currency: walletCurrency.value || undefined,
    wallet_type: walletType.value || undefined,
  };

  const success = await walletStore.createWallet(wallet);
  if (success) {
    showCreateDialog.value = false;
  }
}

// Open edit dialog
function openEditDialog(wallet: any) {
  editingWalletId.value = wallet.id;
  walletName.value = wallet.name;
  walletBalance.value = parseFloat(wallet.balance);
  walletCurrency.value = wallet.currency;
  walletType.value = wallet.wallet_type;
  showEditDialog.value = true;
}

// Handle edit wallet
async function handleEdit() {
  if (!walletName.value.trim()) {
    return;
  }

  const updates: UpdateWallet = {
    name: walletName.value,
    balance: walletBalance.value ?? undefined,
    currency: walletCurrency.value || undefined,
    wallet_type: walletType.value || undefined,
  };

  const success = await walletStore.updateWallet(editingWalletId.value, updates);
  if (success) {
    showEditDialog.value = false;
  }
}

// Open delete dialog
function openDeleteDialog(walletId: string) {
  deletingWalletId.value = walletId;
  showDeleteDialog.value = true;
}

// Handle delete wallet
async function handleDelete() {
  const success = await walletStore.deleteWallet(deletingWalletId.value);
  if (success) {
    showDeleteDialog.value = false;
  }
}

// Format balance for display
function formatBalance(balance: string, currency: string): string {
  const amount = parseFloat(balance);
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: currency.toUpperCase(),
  }).format(amount);
}

// Get wallet type icon
function getWalletIcon(type: string): string {
  const icons: Record<string, string> = {
    checking: "pi pi-credit-card",
    savings: "pi pi-money-bill",
    cash: "pi pi-dollar",
    credit: "pi pi-credit-card",
    general: "pi pi-wallet",
  };
  return icons[type] || "pi pi-wallet";
}
</script>

<template>
  <div className="container mx-auto px-4 py-8">
    <div className="flex justify-between items-center mb-6">
      <h1 className="text-3xl font-bold text-white">{{ t("wallets.title") }}</h1>
      <Button
        :label="t('wallets.create_wallet')"
        icon="pi pi-plus"
        @click="openCreateDialog"
        :pt="glassButtonsStyles.selectedButtonPt"
      />
    </div>

    <!-- Loading state -->
    <div v-if="walletStore.loading && walletStore.wallets.length === 0" className="text-center py-12">
      <ProgressSpinner />
    </div>

    <!-- Empty state -->
    <div
      v-else-if="walletStore.wallets.length === 0"
      className="text-center py-12 backdrop-blur-2xl bg-white/10 border border-white/20 rounded-2xl p-8"
    >
      <i className="pi pi-wallet text-6xl text-white/50 mb-4"></i>
      <h2 className="text-xl text-white mb-2">{{ t("wallets.no_wallets") }}</h2>
      <p className="text-white/70 mb-4">{{ t("wallets.no_wallets_description") }}</p>
      <Button
        :label="t('wallets.create_first_wallet')"
        icon="pi pi-plus"
        @click="openCreateDialog"
        :pt="glassButtonsStyles.selectedButtonPt"
      />
    </div>

    <!-- Wallets grid -->
    <div v-else className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div
        v-for="wallet in walletStore.wallets"
        :key="wallet.id"
        className="backdrop-blur-2xl bg-white/10 border border-white/20 rounded-2xl p-6 hover:bg-white/15 transition-all"
      >
        <div className="flex justify-between items-start mb-4">
          <div className="flex items-center gap-3">
            <i :class="getWalletIcon(wallet.wallet_type)" className="text-2xl text-white"></i>
            <div>
              <h3 className="text-lg font-semibold text-white">{{ wallet.name }}</h3>
              <span className="text-sm text-white/60">{{ t(`wallets.types.${wallet.wallet_type}`) }}</span>
            </div>
          </div>
          <div className="flex gap-2">
            <Button
              icon="pi pi-pencil"
              text
              rounded
              @click="openEditDialog(wallet)"
              :pt="glassButtonsStyles.unselectedButtonPt"
            />
            <Button
              icon="pi pi-trash"
              text
              rounded
              severity="danger"
              @click="openDeleteDialog(wallet.id)"
              :pt="glassButtonsStyles.unselectedButtonPt"
            />
          </div>
        </div>
        <div className="text-3xl font-bold text-white mb-2">
          {{ formatBalance(wallet.balance, wallet.currency) }}
        </div>
        <div className="text-sm text-white/50">
          {{ t("wallets.last_updated") }}: {{ new Date(wallet.updated_at).toLocaleDateString() }}
        </div>
      </div>
    </div>

    <!-- Create Dialog -->
    <Dialog
      v-model:visible="showCreateDialog"
      :header="t('wallets.create_wallet')"
      :modal="true"
      :style="{ width: '450px' }"
    >
      <div className="flex flex-col gap-4 py-4">
        <div className="flex flex-col gap-2">
          <label htmlFor="wallet-name" className="font-semibold">{{ t("wallets.wallet_name") }}</label>
          <InputText
            id="wallet-name"
            v-model="walletName"
            :placeholder="t('wallets.enter_wallet_name')"
            autofocus
          />
        </div>
        <div className="flex flex-col gap-2">
          <label htmlFor="wallet-balance" className="font-semibold">{{ t("wallets.initial_balance") }}</label>
          <InputNumber
            id="wallet-balance"
            v-model="walletBalance"
            :placeholder="t('wallets.enter_balance')"
            mode="decimal"
            :minFractionDigits="2"
            :maxFractionDigits="2"
          />
        </div>
        <div className="flex flex-col gap-2">
          <label htmlFor="wallet-currency" className="font-semibold">{{ t("wallets.currency") }}</label>
          <InputText
            id="wallet-currency"
            v-model="walletCurrency"
            :placeholder="t('wallets.currency_code')"
            maxlength="3"
          />
        </div>
        <div className="flex flex-col gap-2">
          <label htmlFor="wallet-type" className="font-semibold">{{ t("wallets.wallet_type") }}</label>
          <Select
            id="wallet-type"
            v-model="walletType"
            :options="walletTypes"
            optionLabel="label"
            optionValue="value"
            :placeholder="t('wallets.select_type')"
          />
        </div>
      </div>
      <template #footer>
        <Button
          :label="t('wallets.cancel')"
          severity="secondary"
          @click="showCreateDialog = false"
          :pt="glassButtonsStyles.unselectedButtonPt"
        />
        <Button
          :label="t('wallets.create')"
          @click="handleCreate"
          :loading="walletStore.loading"
          :pt="glassButtonsStyles.selectedButtonPt"
        />
      </template>
    </Dialog>

    <!-- Edit Dialog -->
    <Dialog
      v-model:visible="showEditDialog"
      :header="t('wallets.edit_wallet')"
      :modal="true"
      :style="{ width: '450px' }"
    >
      <div className="flex flex-col gap-4 py-4">
        <div className="flex flex-col gap-2">
          <label htmlFor="edit-wallet-name" className="font-semibold">{{ t("wallets.wallet_name") }}</label>
          <InputText
            id="edit-wallet-name"
            v-model="walletName"
            :placeholder="t('wallets.enter_wallet_name')"
          />
        </div>
        <div className="flex flex-col gap-2">
          <label htmlFor="edit-wallet-balance" className="font-semibold">{{ t("wallets.balance") }}</label>
          <InputNumber
            id="edit-wallet-balance"
            v-model="walletBalance"
            :placeholder="t('wallets.enter_balance')"
            mode="decimal"
            :minFractionDigits="2"
            :maxFractionDigits="2"
          />
        </div>
        <div className="flex flex-col gap-2">
          <label htmlFor="edit-wallet-currency" className="font-semibold">{{ t("wallets.currency") }}</label>
          <InputText
            id="edit-wallet-currency"
            v-model="walletCurrency"
            :placeholder="t('wallets.currency_code')"
            maxlength="3"
          />
        </div>
        <div className="flex flex-col gap-2">
          <label htmlFor="edit-wallet-type" className="font-semibold">{{ t("wallets.wallet_type") }}</label>
          <Select
            id="edit-wallet-type"
            v-model="walletType"
            :options="walletTypes"
            optionLabel="label"
            optionValue="value"
            :placeholder="t('wallets.select_type')"
          />
        </div>
      </div>
      <template #footer>
        <Button
          :label="t('wallets.cancel')"
          severity="secondary"
          @click="showEditDialog = false"
          :pt="glassButtonsStyles.unselectedButtonPt"
        />
        <Button
          :label="t('wallets.save')"
          @click="handleEdit"
          :loading="walletStore.loading"
          :pt="glassButtonsStyles.selectedButtonPt"
        />
      </template>
    </Dialog>

    <!-- Delete Dialog -->
    <Dialog
      v-model:visible="showDeleteDialog"
      :header="t('wallets.delete_wallet')"
      :modal="true"
      :style="{ width: '450px' }"
    >
      <div className="py-4">
        <p>{{ t("wallets.delete_confirmation") }}</p>
      </div>
      <template #footer>
        <Button
          :label="t('wallets.cancel')"
          severity="secondary"
          @click="showDeleteDialog = false"
          :pt="glassButtonsStyles.unselectedButtonPt"
        />
        <Button
          :label="t('wallets.delete')"
          severity="danger"
          @click="handleDelete"
          :loading="walletStore.loading"
          :pt="glassButtonsStyles.selectedButtonPt"
        />
      </template>
    </Dialog>
  </div>
</template>
