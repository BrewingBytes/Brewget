<script setup lang="ts">
import type { Transaction } from "@/services/transaction/types";
import { useWalletStore } from "@/stores/wallet";
import { computed } from "vue";
import { useI18n } from "vue-i18n";

interface Props {
  transaction: Transaction;
}

interface Emits {
  (event: "edit", transaction: Transaction): void;
  (event: "delete", transaction: Transaction): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();
const { t } = useI18n();

const walletStore = useWalletStore();

const formatCurrency = (amount: number) => {
  const wallet = walletStore.wallets.find((w) => w.id === props.transaction.wallet_id);
  const currency = wallet?.currency || "USD";
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: currency,
  }).format(amount);
};

const formatDate = (date: string) => {
  return new Intl.DateTimeFormat("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  }).format(new Date(date));
};

const transactionIcon = computed(() => {
  switch (props.transaction.transaction_type) {
    case "Income":
      return "pi-arrow-down";
    case "Expense":
      return "pi-arrow-up";
    case "Transfer":
      return "pi-arrows-h";
    default:
      return "pi-circle";
  }
});

const transactionIconColor = computed(() => {
  switch (props.transaction.transaction_type) {
    case "Income":
      return "text-green-400";
    case "Expense":
      return "text-red-400";
    case "Transfer":
      return "text-blue-400";
    default:
      return "text-white";
  }
});

const categoryColor = computed(() => {
  const colors: Record<string, string> = {
    salary: "bg-green-500/20 text-green-300 border-green-400/50",
    freelance: "bg-blue-500/20 text-blue-300 border-blue-400/50",
    investment: "bg-purple-500/20 text-purple-300 border-purple-400/50",
    gift: "bg-pink-500/20 text-pink-300 border-pink-400/50",
    other_income: "bg-teal-500/20 text-teal-300 border-teal-400/50",
    food: "bg-orange-500/20 text-orange-300 border-orange-400/50",
    transportation: "bg-yellow-500/20 text-yellow-300 border-yellow-400/50",
    housing: "bg-indigo-500/20 text-indigo-300 border-indigo-400/50",
    utilities: "bg-cyan-500/20 text-cyan-300 border-cyan-400/50",
    healthcare: "bg-red-500/20 text-red-300 border-red-400/50",
    entertainment: "bg-violet-500/20 text-violet-300 border-violet-400/50",
    shopping: "bg-fuchsia-500/20 text-fuchsia-300 border-fuchsia-400/50",
    education: "bg-blue-500/20 text-blue-300 border-blue-400/50",
    travel: "bg-emerald-500/20 text-emerald-300 border-emerald-400/50",
    insurance: "bg-slate-500/20 text-slate-300 border-slate-400/50",
    other_expense: "bg-gray-500/20 text-gray-300 border-gray-400/50",
  };
  return colors[props.transaction.category] || "bg-gray-500/20 text-gray-300 border-gray-400/50";
});

const getWalletName = (walletId: string) => {
  const wallet = walletStore.wallets.find((w) => w.id === walletId);
  return wallet?.name || "Unknown";
};
</script>

<template>
  <Card class="backdrop-blur-xl! bg-white/10! border! border-white/30! shadow-xl!">
    <template #content>
      <div class="flex items-start justify-between text-white">
        <div class="flex items-start gap-4 flex-1">
          <!-- Icon -->
          <div class="pt-1">
            <i :class="[transactionIcon, transactionIconColor, 'text-2xl']"></i>
          </div>

          <!-- Content -->
          <div class="flex-1 space-y-2">
            <div class="flex items-start justify-between gap-4">
              <div class="flex-1">
                <div class="flex items-center gap-2 mb-1">
                  <span class="font-semibold text-lg">{{ t(`transactions.types.${transaction.transaction_type.toLowerCase()}`) }}</span>
                  <span :class="['px-2 py-1 rounded-full text-xs border', categoryColor]">
                    {{ t(`transactions.categories.${transaction.category}`) }}
                  </span>
                </div>
                <p v-if="transaction.description" class="text-white/70 text-sm">{{ transaction.description }}</p>
              </div>
              <div class="text-right">
                <p class="text-2xl font-bold">{{ formatCurrency(transaction.amount) }}</p>
              </div>
            </div>

            <div class="flex items-center gap-4 text-sm text-white/60">
              <div class="flex items-center gap-1">
                <i class="pi pi-wallet"></i>
                <span v-if="transaction.transaction_type === 'Transfer' && transaction.destination_wallet_id">
                  {{ getWalletName(transaction.wallet_id) }} → {{ getWalletName(transaction.destination_wallet_id) }}
                </span>
                <span v-else>{{ getWalletName(transaction.wallet_id) }}</span>
              </div>
              <div class="flex items-center gap-1">
                <i class="pi pi-calendar"></i>
                <span>{{ formatDate(transaction.transaction_date) }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Actions -->
        <div class="flex gap-2 ml-4">
          <Button icon="pi pi-pencil" text rounded @click="emit('edit', transaction)"
            class="text-white! hover:bg-white/20!" />
          <Button icon="pi pi-trash" text rounded severity="danger" @click="emit('delete', transaction)"
            class="text-red-300! hover:bg-red-500/20!" />
        </div>
      </div>
    </template>
  </Card>
</template>
