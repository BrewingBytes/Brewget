<script setup lang="ts">
interface Props {
  label?: string;
  icon?: string;
  loading?: boolean;
  severity?: "secondary" | "danger" | "success" | "info" | "warning" | "help" | "contrast" | undefined;
  text?: boolean;
  rounded?: boolean;
  class?: string;
}

interface Emits {
  (event: "click"): void;
}

withDefaults(defineProps<Props>(), {
  label: "",
  icon: "",
  loading: false,
  severity: undefined,
  text: false,
  rounded: false,
  class: "",
});

const emit = defineEmits<Emits>();
</script>

<template>
  <Button :label="label" :icon="icon" :loading="loading" :severity="severity" :text="text" :rounded="rounded"
    @click="emit('click')" :class="`bg-white/10! border-white! text-white! hover:bg-white/20! ${$props.class}`" />
</template>
