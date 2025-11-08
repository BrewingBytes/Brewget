<script setup lang="ts">
interface Props {
  modelValue: string;
  options: string[] | { label: string; value: string }[];
  optionLabel?: string;
  optionValue?: string;
  placeholder?: string;
  editable?: boolean;
  class?: string;
}

interface Emits {
  (event: "update:modelValue", value: string): void;
}

withDefaults(defineProps<Props>(), {
  optionLabel: "",
  optionValue: "",
  placeholder: "",
  editable: false,
  class: "",
});

const emit = defineEmits<Emits>();

const handleChange = (value: string) => {
  emit("update:modelValue", value);
};
</script>

<template>
  <Select :modelValue="modelValue" @update:modelValue="handleChange" :options="options" :optionLabel="optionLabel"
    :optionValue="optionValue" :placeholder="placeholder" :editable="editable"
    :class="`w-full bg-transparent! border-white! ${$props.class}`" :pt="{
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
</template>
