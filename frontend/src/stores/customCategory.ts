import { defineStore } from "pinia";
import { ref } from "vue";

import { useToastStore } from "./toast";

import type {
  CreateCustomCategory,
  CustomCategory,
  UpdateCustomCategory,
} from "@/services/transaction/types";

import i18n from "@/i18n";
import { transactionService } from "@/services/transaction";
import { ServerStatus } from "@/services/types";

export const useCustomCategoryStore = defineStore("customCategory", () => {
  const categories = ref<CustomCategory[]>([]);
  const loading = ref(false);

  async function loadCategories(): Promise<void> {
    loading.value = true;
    try {
      const response = await transactionService.getCategories();

      if (response.status !== ServerStatus.NO_ERROR) {
        useToastStore().showError(i18n.global.t("categories.failed_to_load"));
        return;
      }

      categories.value = response.data;
    } finally {
      loading.value = false;
    }
  }

  async function createCategory(category: CreateCustomCategory): Promise<boolean> {
    loading.value = true;
    try {
      const response = await transactionService.createCategory(category);

      if (response.status === ServerStatus.NO_ERROR || response.status === ServerStatus.CREATED) {
        categories.value.unshift(response.data as CustomCategory);
        useToastStore().showInfo(i18n.global.t("categories.category_created"));
        return true;
      }

      useToastStore().showError(i18n.global.t("categories.failed_to_create"));
      return false;
    } finally {
      loading.value = false;
    }
  }

  async function updateCategory(
    id: string,
    category: UpdateCustomCategory,
  ): Promise<boolean> {
    loading.value = true;
    try {
      const response = await transactionService.updateCategory(id, category);

      if (response.status !== ServerStatus.NO_ERROR) {
        useToastStore().showError(i18n.global.t("categories.failed_to_update"));
        return false;
      }

      const index = categories.value.findIndex((c) => c.id === id);
      if (index !== -1) {
        categories.value[index] = response.data;
      }
      useToastStore().showInfo(i18n.global.t("categories.category_updated"));
      return true;
    } finally {
      loading.value = false;
    }
  }

  async function deleteCategory(id: string): Promise<boolean> {
    loading.value = true;
    try {
      const response = await transactionService.deleteCategory(id);

      if (response.status !== ServerStatus.NO_ERROR && response.status !== ServerStatus.NO_CONTENT) {
        useToastStore().showError(i18n.global.t("categories.failed_to_delete"));
        return false;
      }

      categories.value = categories.value.filter((c) => c.id !== id);
      useToastStore().showInfo(i18n.global.t("categories.category_deleted"));
      return true;
    } finally {
      loading.value = false;
    }
  }

  return {
    categories,
    loading,
    loadCategories,
    createCategory,
    updateCategory,
    deleteCategory,
  };
});
