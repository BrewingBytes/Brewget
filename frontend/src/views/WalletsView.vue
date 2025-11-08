<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";

import type { CreateWallet, UpdateWallet, Wallet } from "@/services/transaction/types";

import { useWalletStore } from "@/stores/wallet";

const { t } = useI18n();
const walletStore = useWalletStore();

const showCreateDialog = ref(false);
const showEditDialog = ref(false);
const showDeleteDialog = ref(false);
const selectedWallet = ref<Wallet | null>(null);

const newWallet = ref<CreateWallet>({
  name: "",
  balance: 0,
  currency: "USD",
});

const editWallet = ref<UpdateWallet>({
  name: "",
  balance: 0,
  currency: "USD",
});

onMounted(async () => {
  await walletStore.loadWallets();
});

const openCreateDialog = () => {
  newWallet.value = {
    name: "",
    balance: 0,
    currency: "USD",
  };
  showCreateDialog.value = true;
};

const createWallet = async () => {
  const success = await walletStore.createWallet(newWallet.value);
  if (success) {
    showCreateDialog.value = false;
  }
};

const openEditDialog = (wallet: Wallet) => {
  selectedWallet.value = wallet;
  editWallet.value = {
    name: wallet.name,
    balance: wallet.balance,
    currency: wallet.currency,
  };
  showEditDialog.value = true;
};

const updateWallet = async () => {
  if (selectedWallet.value) {
    const success = await walletStore.updateWallet(selectedWallet.value.id, editWallet.value);
    if (success) {
      showEditDialog.value = false;
      selectedWallet.value = null;
    }
  }
};

const openDeleteDialog = (wallet: Wallet) => {
  selectedWallet.value = wallet;
  showDeleteDialog.value = true;
};

const deleteWallet = async () => {
  if (selectedWallet.value) {
    const success = await walletStore.deleteWallet(selectedWallet.value.id);
    if (success) {
      showDeleteDialog.value = false;
      selectedWallet.value = null;
    }
  }
};

const formatCurrency = (amount: number, currency: string) => {
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: currency,
  }).format(amount);
};

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleDateString();
};
</script>

<template>
  <div class="container mx-auto p-6">
    <div class="flex justify-between items-center mb-6">
      <h1 class="text-3xl font-bold">{{ t("wallets.title") }}</h1>
      <Button
        :label="t('wallets.create_wallet')"
        icon="pi pi-plus"
        @click="openCreateDialog"
      />
    </div>

    <div v-if="walletStore.loading" class="text-center py-8">
      <ProgressSpinner />
    </div>

    <div v-else-if="walletStore.wallets.length === 0" class="text-center py-8">
      <p class="text-gray-500 mb-4">{{ t("wallets.no_wallets") }}</p>
      <Button
        :label="t('wallets.create_wallet')"
        icon="pi pi-plus"
        @click="openCreateDialog"
      />
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      <Card v-for="wallet in walletStore.wallets" :key="wallet.id" class="shadow-lg">
        <template #title>
          <div class="flex justify-between items-center">
            <span>{{ wallet.name }}</span>
            <div class="flex gap-2">
              <Button
                icon="pi pi-pencil"
                text
                rounded
                @click="openEditDialog(wallet)"
              />
              <Button
                icon="pi pi-trash"
                text
                rounded
                severity="danger"
                @click="openDeleteDialog(wallet)"
              />
            </div>
          </div>
        </template>
        <template #content>
          <div class="space-y-2">
            <div>
              <p class="text-2xl font-bold">
                {{ formatCurrency(wallet.balance, wallet.currency) }}
              </p>
              <p class="text-sm text-gray-500">{{ wallet.currency }}</p>
            </div>
            <div class="text-sm text-gray-500">
              <p>{{ t("wallets.created_at") }}: {{ formatDate(wallet.created_at) }}</p>
              <p>{{ t("wallets.updated_at") }}: {{ formatDate(wallet.updated_at) }}</p>
            </div>
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
          <label class="block mb-2">{{ t("wallets.wallet_name") }}</label>
          <InputText
            v-model="newWallet.name"
            :placeholder="t('wallets.enter_wallet_name')"
            class="w-full"
          />
        </div>
        <div>
          <label class="block mb-2">{{ t("wallets.balance") }}</label>
          <InputNumber
            v-model="newWallet.balance"
            :placeholder="t('wallets.enter_balance')"
            class="w-full"
            mode="decimal"
            :minFractionDigits="2"
            :maxFractionDigits="2"
          />
        </div>
        <div>
          <label class="block mb-2">{{ t("wallets.currency") }}</label>
          <Select
            v-model="newWallet.currency"
            :options="['USD', 'EUR', 'GBP', 'CAD', 'JPY']"
            class="w-full"
          />
        </div>
      </div>
      <template #footer>
        <Button
          :label="t('settings.save_settings')"
          icon="pi pi-check"
          @click="createWallet"
          :loading="walletStore.loading"
        />
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
          <label class="block mb-2">{{ t("wallets.wallet_name") }}</label>
          <InputText
            v-model="editWallet.name"
            :placeholder="t('wallets.enter_wallet_name')"
            class="w-full"
          />
        </div>
        <div>
          <label class="block mb-2">{{ t("wallets.balance") }}</label>
          <InputNumber
            v-model="editWallet.balance"
            :placeholder="t('wallets.enter_balance')"
            class="w-full"
            mode="decimal"
            :minFractionDigits="2"
            :maxFractionDigits="2"
          />
        </div>
        <div>
          <label class="block mb-2">{{ t("wallets.currency") }}</label>
          <Select
            v-model="editWallet.currency"
            :options="['USD', 'EUR', 'GBP', 'CAD', 'JPY']"
            class="w-full"
          />
        </div>
      </div>
      <template #footer>
        <Button
          :label="t('settings.save_settings')"
          icon="pi pi-check"
          @click="updateWallet"
          :loading="walletStore.loading"
        />
      </template>
    </Dialog>

    <!-- Delete Wallet Dialog -->
    <Dialog
      v-model:visible="showDeleteDialog"
      :header="t('wallets.delete_wallet')"
      :modal="true"
      :closable="true"
      class="w-full max-w-md"
    >
      <p>{{ t("wallets.confirm_delete") }}</p>
      <template #footer>
        <Button
          :label="t('auth.cancel')"
          icon="pi pi-times"
          text
          @click="showDeleteDialog = false"
        />
        <Button
          :label="t('wallets.delete_wallet')"
          icon="pi pi-trash"
          severity="danger"
          @click="deleteWallet"
          :loading="walletStore.loading"
        />
      </template>
    </Dialog>
  </div>
</template>
