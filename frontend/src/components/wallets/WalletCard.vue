<script setup lang="ts">
import { useRouter } from "vue-router";

import type { Wallet } from "@/services/transaction/types";

interface Props {
  wallet: Wallet;
}

interface Emits {
  (event: "edit", wallet: Wallet): void;
  (event: "delete", wallet: Wallet): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();
const router = useRouter();

const formatCurrency = (amount: number, currency: string) => {
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: currency,
  }).format(amount);
};

const viewTransactions = (walletId: string) => {
  router.push({ name: "transactions", query: { wallet: walletId } });
};
</script>

<template>
  <Card class="backdrop-blur-xl! bg-white/10! border! border-white/30! shadow-xl!">
    <template #title>
      <div class="flex justify-between items-center text-white">
        <div class="flex items-center gap-2">
          <i class="pi pi-wallet pr-2"></i>
          <span>{{ wallet.name }}</span>
        </div>
        <div class="flex gap-2">
          <Button
            icon="pi pi-pencil"
            text
            rounded
            @click="emit('edit', wallet)"
            class="text-white! hover:bg-white/20!"
          />
          <Button
            icon="pi pi-trash"
            text
            rounded
            severity="danger"
            @click="emit('delete', wallet)"
            class="text-red-300! hover:bg-red-500/20!"
          />
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
        <div class="pt-2">
          <Button
            label="View Transactions"
            icon="pi pi-list"
            text
            @click="viewTransactions(wallet.id)"
            class="text-white! hover:bg-white/20! w-full justify-center"
          />
        </div>
      </div>
    </template>
  </Card>
</template>
