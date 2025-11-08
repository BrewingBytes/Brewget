<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";

import type { CreateWallet, UpdateWallet, Wallet } from "@/services/transaction/types";

import { useToastStore } from "@/stores/toast";
import { useWalletStore } from "@/stores/wallet";

const { t } = useI18n();
const walletStore = useWalletStore();

const showCreateDialog = ref(false);
const showEditDialog = ref(false);
const showDeleteDialog = ref(false);
const selectedWallet = ref<Wallet | null>(null);

const newWalletBalance = ref("");

const newWallet = ref<CreateWallet>({
  name: "",
  balance: 0,
  currency: "USD",
  category: "",
});

const editWallet = ref<UpdateWallet>({
  name: "",
  currency: "USD",
  category: "",
});

const currencyOptions = ["USD", "EUR", "GBP", "CAD", "JPY", "RON"];

// Group wallets by category
const walletsByCategory = computed(() => {
  const grouped = new Map<string, Wallet[]>();
  
  walletStore.wallets.forEach((wallet) => {
    const category = wallet.category || t("wallets.uncategorized");
    if (!grouped.has(category)) {
      grouped.set(category, []);
    }
    grouped.get(category)!.push(wallet);
  });
  
  return Array.from(grouped.entries()).map(([category, wallets]) => ({
    category,
    wallets,
  }));
});

onMounted(async () => {
  await walletStore.loadWallets();
});

const validateBalanceInput = (event: Event) => {
  const input = event.target as HTMLInputElement;
  let value = input.value;

  // Remove any non-numeric characters except decimal point
  value = value.replace(/[^0-9.]/g, "");

  // Ensure only one decimal point
  const parts = value.split(".");
  if (parts.length > 2) {
    value = `${parts[0]}.${parts.slice(1).join("")}`;
  }

  // Limit to 2 decimal places
  if (parts.length === 2 && parts[1] && parts[1].length > 2) {
    value = `${parts[0]}.${parts[1].substring(0, 2)}`;
  }

  input.value = value;
};

const openCreateDialog = () => {
  newWallet.value = {
    name: "",
    balance: 0,
    currency: "USD",
    category: "",
  };
  newWalletBalance.value = "";
  showCreateDialog.value = true;
};

const createWallet = async () => {
  // Validate name is required
  if (!newWallet.value.name || newWallet.value.name.trim() === "") {
    useToastStore().showError(t("wallets.name_required"));
    return;
  }

  // Parse balance from string, default to 0 if empty
  const balance = newWalletBalance.value ? parseFloat(newWalletBalance.value) : 0;
  newWallet.value.balance = balance;
  
  const success = await walletStore.createWallet(newWallet.value);
  if (success) {
    showCreateDialog.value = false;
  }
};

const openEditDialog = (wallet: Wallet) => {
  selectedWallet.value = wallet;
  editWallet.value = {
    name: wallet.name,
    currency: wallet.currency,
    category: wallet.category || "",
  };
  showEditDialog.value = true;
};

const updateWallet = async () => {
  if (selectedWallet.value) {
    // Validate name is required
    if (!editWallet.value.name || editWallet.value.name.trim() === "") {
      useToastStore().showError(t("wallets.name_required"));
      return;
    }
    
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

</script>

<template>
  <div class="flex items-center justify-center min-h-screen p-4 bg-gradient-to-b from-blue-300 to-blue-500">
    <div class="w-full max-w-6xl">
      <Card class="backdrop-blur-2xl! bg-transparent! border! border-white/80! shadow-2xl! mb-6">
        <template #title>
          <div class="flex items-center justify-between text-white">
            <div class="flex items-center gap-3">
              <i class="pi pi-wallet text-2xl"></i>
              <span class="text-2xl font-medium">{{ t("wallets.title") }}</span>
            </div>
            <Button :label="t('wallets.create_wallet')" icon="pi pi-plus" @click="openCreateDialog"
              class="bg-white/10! border-white! text-white! hover:bg-white/20!" />
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
            <Button :label="t('wallets.create_wallet')" icon="pi pi-plus" @click="openCreateDialog"
              class="bg-white/10! border-white! text-white! hover:bg-white/20!" />
          </div>

          <div v-else class="space-y-6">
            <div v-for="group in walletsByCategory" :key="group.category" class="space-y-3">
              <h3 class="text-xl font-semibold text-white/90 flex items-center gap-2">
                <i class="pi pi-folder"></i>
                {{ group.category }}
              </h3>
              <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                <Card v-for="wallet in group.wallets" :key="wallet.id"
                  class="backdrop-blur-xl! bg-white/10! border! border-white/30! shadow-xl!">
                  <template #title>
                    <div class="flex justify-between items-center text-white">
                      <div class="flex items-center gap-2">
                        <i class="pi pi-wallet pr-2"></i>
                        <span>{{ wallet.name }}</span>
                      </div>
                      <div class="flex gap-2">
                        <Button icon="pi pi-pencil" text rounded @click="openEditDialog(wallet)"
                          class="text-white! hover:bg-white/20!" />
                        <Button icon="pi pi-trash" text rounded severity="danger" @click="openDeleteDialog(wallet)"
                          class="text-red-300! hover:bg-red-500/20!" />
                      </div>
                    </div>
                  </template>
                  <template #content>
                    <div class="space-y-3 text-white">
                      <div>
                        <p class="text-3xl font-bold">
                          {{ formatCurrency(wallet.balance, wallet.currency) }}
                        </p>
                      </div>
                    </div>
                  </template>
                </Card>
              </div>
            </div>
          </div>
        </template>
      </Card>
    </div>

    <!-- Create Wallet Dialog -->
    <Dialog v-model:visible="showCreateDialog" :header="t('wallets.create_wallet')" :modal="true" :closable="true"
      class="w-full max-w-md" :pt="{
        root: {
          class: 'backdrop-blur-2xl! bg-transparent! border! border-white/20! shadow-2xl!',
        },
        header: {
          class: 'bg-transparent! border-b! border-white/20! text-white!',
        },
        content: {
          class: 'bg-transparent! text-white!',
        },
        footer: {
          class: 'bg-transparent!',
        },
      }" pt:mask:class="backdrop-blur-xs! bg-transparent!">
      <div class="space-y-4">
        <div>
          <label class="block mb-2 text-white/90"><i class="pi pi-tag mr-2"></i>{{ t("wallets.wallet_name")
            }}</label>
          <InputText v-model="newWallet.name" :placeholder="t('wallets.enter_wallet_name')"
            class="w-full bg-transparent! border-white! text-white!" />
        </div>
        <div>
          <label class="block mb-2 text-white/90"><i class="pi pi-dollar mr-2"></i>{{ t("wallets.initial_balance")
            }}</label>
          <InputText v-model="newWalletBalance" :placeholder="t('wallets.enter_balance')" @input="validateBalanceInput"
            class="w-full bg-transparent! border-white! text-white!" />
        </div>
        <div>
          <label class="block mb-2 text-white/90"><i class="pi pi-money-bill mr-2"></i>{{ t("wallets.currency")
            }}</label>
          <Select v-model="newWallet.currency" :options="currencyOptions" class="w-full bg-transparent! border-white!"
            :pt="{
              label: {
                class: 'text-white/90!',
              },
              overlay: {
                class: 'bg-transparent! border-white! backdrop-blur-xs!',
              },
              option: {
                class: 'text-white/90! bg-transparent! hover:bg-white/10!',
              },
            }">
            <template #dropdownicon>
              <i class="pi pi-chevron-down text-white" />
            </template>
          </Select>
        </div>
        <div>
          <label class="block mb-2 text-white/90"><i class="pi pi-folder mr-2"></i>{{ t("wallets.category")
            }}</label>
          <InputText v-model="newWallet.category" :placeholder="t('wallets.enter_category')"
            class="w-full bg-transparent! border-white! text-white!" />
        </div>
      </div>
      <template #footer>
        <Button :label="t('settings.save_settings')" icon="pi pi-check" @click="createWallet"
          :loading="walletStore.loading" class="bg-white/10! border-white! text-white! hover:bg-white/20!" />
      </template>
    </Dialog>

    <!-- Edit Wallet Dialog -->
    <Dialog v-model:visible="showEditDialog" :header="t('wallets.edit_wallet')" :modal="true" :closable="true"
      class="w-full max-w-md" :pt="{
        root: {
          class: 'backdrop-blur-2xl! bg-transparent! border! border-white/20! shadow-2xl!',
        },
        header: {
          class: 'bg-transparent! border-b! border-white/20! text-white!',
        },
        content: {
          class: 'bg-transparent! text-white!',
        },
        footer: {
          class: 'bg-transparent!',
        },
      }" pt:mask:class="backdrop-blur-xs! bg-transparent!">
      <div class="space-y-4">
        <div>
          <label class="block mb-2 text-white/90"><i class="pi pi-tag mr-2"></i>{{ t("wallets.wallet_name") }}</label>
          <InputText v-model="editWallet.name" :placeholder="t('wallets.enter_wallet_name')"
            class="w-full bg-transparent! border-white! text-white!" />
        </div>
        <div>
          <label class="block mb-2 text-white/90"><i class="pi pi-money-bill mr-2"></i>{{ t("wallets.currency")
            }}</label>
          <Select v-model="editWallet.currency" :options="currencyOptions" class="w-full bg-transparent! border-white!"
            :pt="{
              label: {
                class: 'text-white/90!',
              },
              overlay: {
                class: 'bg-transparent! border-white! backdrop-blur-xs!',
              },
              option: {
                class: 'text-white/90! bg-transparent! hover:bg-white/10!',
              },
            }">
            <template #dropdownicon>
              <i class="pi pi-chevron-down text-white" />
            </template>
          </Select>
        </div>
        <div>
          <label class="block mb-2 text-white/90"><i class="pi pi-folder mr-2"></i>{{ t("wallets.category")
            }}</label>
          <InputText v-model="editWallet.category" :placeholder="t('wallets.enter_category')"
            class="w-full bg-transparent! border-white! text-white!" />
        </div>
      </div>
      <template #footer>
        <Button :label="t('settings.save_settings')" icon="pi pi-check" @click="updateWallet"
          :loading="walletStore.loading" class="bg-white/10! border-white! text-white! hover:bg-white/20!" />
      </template>
    </Dialog>

    <!-- Delete Wallet Dialog -->
    <Dialog v-model:visible="showDeleteDialog" :header="t('wallets.delete_wallet')" :modal="true" :closable="true"
      class="w-full max-w-md" :pt="{
        root: {
          class: 'backdrop-blur-2xl! bg-transparent! border! border-white/20! shadow-2xl!',
        },
        header: {
          class: 'bg-transparent! border-b! border-white/20! text-white!',
        },
        content: {
          class: 'bg-transparent! text-white!',
        },
        footer: {
          class: 'bg-transparent!',
        },
      }" pt:mask:class="backdrop-blur-xs! bg-transparent!">
      <p class="text-white/90">{{ t("wallets.confirm_delete") }}</p>
      <template #footer>
        <Button :label="t('auth.cancel')" icon="pi pi-times" text @click="showDeleteDialog = false"
          class="text-white! hover:bg-white/10!" />
        <Button :label="t('wallets.delete_wallet')" icon="pi pi-trash" severity="danger" @click="deleteWallet"
          :loading="walletStore.loading" class="bg-red-500/20! border-red-300! text-red-300! hover:bg-red-500/30!" />
      </template>
    </Dialog>
  </div>
</template>
