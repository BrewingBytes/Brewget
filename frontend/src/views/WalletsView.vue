<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";

import {
  Currency,
  type CreateWallet,
  type UpdateWallet,
  type Wallet,
  WalletType,
  walletService,
} from "@/services/wallet";
import { ServerStatus } from "@/services/types";
import { ToastSeverity, useToastStore } from "@/stores/toast";

const { t } = useI18n();
const toastStore = useToastStore();

const wallets = ref<Wallet[]>([]);
const loading = ref(true);
const showCreateDialog = ref(false);
const showEditDialog = ref(false);
const showDeleteDialog = ref(false);
const selectedWallet = ref<Wallet | null>(null);

const newWallet = ref<CreateWallet>({
  name: "",
  balance: 0,
  currency: Currency.USD,
  wallet_type: WalletType.GENERAL,
});

const editWallet = ref<UpdateWallet>({
  name: "",
  balance: 0,
  currency: Currency.USD,
  wallet_type: WalletType.GENERAL,
});

const currencyOptions = [
  { label: "USD ($)", value: Currency.USD },
  { label: "EUR (€)", value: Currency.EUR },
  { label: "RON (lei)", value: Currency.RON },
];

const walletTypeOptions = [
  { label: t("wallets.types.general"), value: WalletType.GENERAL },
  { label: t("wallets.types.savings"), value: WalletType.SAVINGS },
  { label: t("wallets.types.business"), value: WalletType.BUSINESS },
  { label: t("wallets.types.personal"), value: WalletType.PERSONAL },
];

async function loadWallets() {
  loading.value = true;
  const response = await walletService.getWallets();

  if (response.status === ServerStatus.NO_ERROR) {
    wallets.value = response.data;
  } else {
    toastStore.show(t("wallets.failed_to_load"), ToastSeverity.ERROR);
  }
  loading.value = false;
}

function openCreateDialog() {
  newWallet.value = {
    name: "",
    balance: 0,
    currency: Currency.USD,
    wallet_type: WalletType.GENERAL,
  };
  showCreateDialog.value = true;
}

async function createWallet() {
  const response = await walletService.createWallet(newWallet.value);

  if (response.status === ServerStatus.NO_ERROR) {
    toastStore.show(t("wallets.wallet_created"), ToastSeverity.SUCCESS);
    showCreateDialog.value = false;
    await loadWallets();
  } else {
    toastStore.show(t("wallets.failed_to_create"), ToastSeverity.ERROR);
  }
}

function openEditDialog(wallet: Wallet) {
  selectedWallet.value = wallet;
  editWallet.value = {
    name: wallet.name,
    balance: wallet.balance,
    currency: wallet.currency,
    wallet_type: wallet.wallet_type,
  };
  showEditDialog.value = true;
}

async function saveWallet() {
  if (!selectedWallet.value) return;

  const response = await walletService.updateWallet(
    selectedWallet.value.id,
    editWallet.value,
  );

  if (response.status === ServerStatus.NO_ERROR) {
    toastStore.show(t("wallets.wallet_updated"), ToastSeverity.SUCCESS);
    showEditDialog.value = false;
    await loadWallets();
  } else {
    toastStore.show(t("wallets.failed_to_update"), ToastSeverity.ERROR);
  }
}

function openDeleteDialog(wallet: Wallet) {
  selectedWallet.value = wallet;
  showDeleteDialog.value = true;
}

async function confirmDelete() {
  if (!selectedWallet.value) return;

  const response = await walletService.deleteWallet(selectedWallet.value.id);

  if (response.status === ServerStatus.NO_ERROR) {
    toastStore.show(t("wallets.wallet_deleted"), ToastSeverity.SUCCESS);
    showDeleteDialog.value = false;
    await loadWallets();
  } else {
    toastStore.show(t("wallets.failed_to_delete"), ToastSeverity.ERROR);
  }
}

function getCurrencySymbol(currency: Currency): string {
  switch (currency) {
    case Currency.USD:
      return "$";
    case Currency.EUR:
      return "€";
    case Currency.RON:
      return "lei";
    default:
      return "";
  }
}

function formatBalance(balance: number, currency: Currency): string {
  const symbol = getCurrencySymbol(currency);
  if (currency === Currency.RON) {
    return `${balance.toFixed(2)} ${symbol}`;
  }
  return `${symbol}${balance.toFixed(2)}`;
}

onMounted(() => {
  loadWallets();
});
</script>

<template>
  <div class="wallets-container p-6 max-w-7xl mx-auto">
    <div class="flex justify-between items-center mb-6">
      <h1 class="text-3xl font-bold text-white">{{ t("wallets.title") }}</h1>
      <Button
        :label="t('wallets.create_wallet')"
        icon="pi pi-plus"
        @click="openCreateDialog"
        severity="success"
      />
    </div>

    <div v-if="loading" class="flex justify-center items-center h-64">
      <ProgressSpinner />
    </div>

    <div v-else-if="wallets.length === 0" class="text-center py-12">
      <p class="text-xl text-white/70 mb-4">{{ t("wallets.no_wallets") }}</p>
      <Button
        :label="t('wallets.create_wallet')"
        icon="pi pi-plus"
        @click="openCreateDialog"
      />
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      <Card
        v-for="wallet in wallets"
        :key="wallet.id"
        class="wallet-card backdrop-blur-lg bg-white/10 border border-white/20"
      >
        <template #title>
          <div class="flex justify-between items-start">
            <span class="text-white">{{ wallet.name }}</span>
            <span
              class="text-xs px-2 py-1 rounded bg-white/20 text-white/90"
            >
              {{ t(`wallets.types.${wallet.wallet_type}`) }}
            </span>
          </div>
        </template>
        <template #content>
          <div class="space-y-2">
            <div class="text-2xl font-bold text-white">
              {{ formatBalance(wallet.balance, wallet.currency) }}
            </div>
            <div class="text-sm text-white/60">
              {{ t("wallets.currency") }}: {{ wallet.currency.toUpperCase() }}
            </div>
          </div>
        </template>
        <template #footer>
          <div class="flex gap-2">
            <Button
              :label="t('wallets.edit_wallet')"
              icon="pi pi-pencil"
              size="small"
              @click="openEditDialog(wallet)"
              text
            />
            <Button
              :label="t('wallets.delete')"
              icon="pi pi-trash"
              size="small"
              severity="danger"
              @click="openDeleteDialog(wallet)"
              text
            />
          </div>
        </template>
      </Card>
    </div>

    <!-- Create Wallet Dialog -->
    <Dialog
      v-model:visible="showCreateDialog"
      :header="t('wallets.create_wallet')"
      :modal="true"
      :closable="true"
      class="w-full max-w-md"
    >
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium mb-2">
            {{ t("wallets.wallet_name") }}
          </label>
          <InputText
            v-model="newWallet.name"
            class="w-full"
            :placeholder="t('wallets.wallet_name')"
          />
        </div>
        <div>
          <label class="block text-sm font-medium mb-2">
            {{ t("wallets.balance") }}
          </label>
          <InputNumber
            v-model="newWallet.balance"
            class="w-full"
            mode="currency"
            :currency="newWallet.currency"
            locale="en-US"
          />
        </div>
        <div>
          <label class="block text-sm font-medium mb-2">
            {{ t("wallets.currency") }}
          </label>
          <Select
            v-model="newWallet.currency"
            :options="currencyOptions"
            optionLabel="label"
            optionValue="value"
            class="w-full"
          />
        </div>
        <div>
          <label class="block text-sm font-medium mb-2">
            {{ t("wallets.wallet_type") }}
          </label>
          <Select
            v-model="newWallet.wallet_type"
            :options="walletTypeOptions"
            optionLabel="label"
            optionValue="value"
            class="w-full"
          />
        </div>
      </div>
      <template #footer>
        <div class="flex justify-end gap-2">
          <Button
            :label="t('wallets.cancel')"
            @click="showCreateDialog = false"
            text
          />
          <Button
            :label="t('wallets.create')"
            @click="createWallet"
            severity="success"
          />
        </div>
      </template>
    </Dialog>

    <!-- Edit Wallet Dialog -->
    <Dialog
      v-model:visible="showEditDialog"
      :header="t('wallets.edit_wallet')"
      :modal="true"
      :closable="true"
      class="w-full max-w-md"
    >
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium mb-2">
            {{ t("wallets.wallet_name") }}
          </label>
          <InputText
            v-model="editWallet.name"
            class="w-full"
            :placeholder="t('wallets.wallet_name')"
          />
        </div>
        <div>
          <label class="block text-sm font-medium mb-2">
            {{ t("wallets.balance") }}
          </label>
          <InputNumber
            v-model="editWallet.balance"
            class="w-full"
            mode="currency"
            :currency="editWallet.currency"
            locale="en-US"
          />
        </div>
        <div>
          <label class="block text-sm font-medium mb-2">
            {{ t("wallets.currency") }}
          </label>
          <Select
            v-model="editWallet.currency"
            :options="currencyOptions"
            optionLabel="label"
            optionValue="value"
            class="w-full"
          />
        </div>
        <div>
          <label class="block text-sm font-medium mb-2">
            {{ t("wallets.wallet_type") }}
          </label>
          <Select
            v-model="editWallet.wallet_type"
            :options="walletTypeOptions"
            optionLabel="label"
            optionValue="value"
            class="w-full"
          />
        </div>
      </div>
      <template #footer>
        <div class="flex justify-end gap-2">
          <Button
            :label="t('wallets.cancel')"
            @click="showEditDialog = false"
            text
          />
          <Button :label="t('wallets.save')" @click="saveWallet" />
        </div>
      </template>
    </Dialog>

    <!-- Delete Confirmation Dialog -->
    <Dialog
      v-model:visible="showDeleteDialog"
      :header="t('wallets.delete_wallet')"
      :modal="true"
      :closable="true"
      class="w-full max-w-md"
    >
      <p>{{ t("wallets.confirm_delete") }}</p>
      <template #footer>
        <div class="flex justify-end gap-2">
          <Button
            :label="t('wallets.cancel')"
            @click="showDeleteDialog = false"
            text
          />
          <Button
            :label="t('wallets.delete')"
            @click="confirmDelete"
            severity="danger"
          />
        </div>
      </template>
    </Dialog>
  </div>
</template>

<style scoped>
.wallets-container {
  min-height: calc(100vh - 120px);
  padding-bottom: 100px;
}

.wallet-card {
  transition: transform 0.2s;
}

.wallet-card:hover {
  transform: translateY(-4px);
}
</style>
