<script setup lang="ts">
import WalletCard from "./WalletCard.vue";

import type { Wallet } from "@/services/transaction/types";

interface Props {
  walletType: string;
  label: string;
  wallets: Wallet[];
}

interface Emits {
  (event: "edit", wallet: Wallet): void;
  (event: "delete", wallet: Wallet): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();
</script>

<template>
  <div class="space-y-3">
    <h3 class="text-xl font-semibold text-white/90 flex items-center gap-2">
      <i class="pi pi-wallet"></i>
      {{ label }}
    </h3>
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      <WalletCard v-for="wallet in wallets" :key="wallet.id" :wallet="wallet" @edit="emit('edit', wallet)"
        @delete="emit('delete', wallet)" />
    </div>
  </div>
</template>
