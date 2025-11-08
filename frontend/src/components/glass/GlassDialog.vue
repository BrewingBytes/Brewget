<script setup lang="ts">
interface Props {
  visible: boolean;
  header: string;
  class?: string;
}

interface Emits {
  (event: "update:visible", value: boolean): void;
}

withDefaults(defineProps<Props>(), {
  class: "w-full max-w-md",
});

const emit = defineEmits<Emits>();
</script>

<template>
  <Dialog :visible="visible" @update:visible="emit('update:visible', $event)" :header="header" :modal="true"
    :closable="true" :class="$props.class" :pt="{
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
    <slot></slot>
    <template #footer>
      <slot name="footer"></slot>
    </template>
  </Dialog>
</template>
