<script setup lang="ts">
import { useAuthStore } from "@/stores/auth";

</script>

<template>
  <Button @click="useAuthStore().logout()" label="logout" />
</template>
