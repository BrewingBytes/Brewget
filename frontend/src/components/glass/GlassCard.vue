<script setup lang="ts">
interface Props {
  class?: string;
}

withDefaults(defineProps<Props>(), {
  class: "",
});
</script>

<template>
  <Card :class="`backdrop-blur-2xl! bg-transparent! border! border-white/80! shadow-2xl! ${$props.class}`">
    <template #title>
      <slot name="title"></slot>
    </template>
    <template #content>
      <slot name="content"></slot>
    </template>
    <template #footer>
      <slot name="footer"></slot>
    </template>
  </Card>
</template>
