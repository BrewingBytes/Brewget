<script setup lang="ts">
interface Props {
  modelValue: string;
  type?: string;
  placeholder?: string;
  class?: string;
}

interface Emits {
  (event: "update:modelValue", value: string): void;
  (event: "input", e: Event): void;
}

withDefaults(defineProps<Props>(), {
  type: "text",
  placeholder: "",
  class: "",
});

const emit = defineEmits<Emits>();

const handleInput = (e: Event) => {
  const target = e.target as HTMLInputElement;
  emit("update:modelValue", target.value);
  emit("input", e);
};
</script>

<template>
  <InputText :modelValue="modelValue" @input="handleInput" :type="type" :placeholder="placeholder"
    :class="`w-full bg-transparent! border-white! text-white! ${$props.class}`" />
</template>
