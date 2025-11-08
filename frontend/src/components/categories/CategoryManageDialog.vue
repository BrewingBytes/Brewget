<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";

import type {
  CreateCustomCategory,
  CustomCategory,
  UpdateCustomCategory,
} from "@/services/transaction/types";

import GlassButton from "@/components/glass/GlassButton.vue";
import GlassDialog from "@/components/glass/GlassDialog.vue";
import GlassDropdown from "@/components/glass/GlassDropdown.vue";
import GlassInput from "@/components/glass/GlassInput.vue";
import { useCustomCategoryStore } from "@/stores/customCategory";
import { useToastStore } from "@/stores/toast";

interface Props {
  visible: boolean;
}

interface Emits {
  (event: "update:visible", value: boolean): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();
const { t } = useI18n();

const categoryStore = useCustomCategoryStore();

const isCreating = ref(false);
const editingCategory = ref<CustomCategory | null>(null);
const newCategory = ref<CreateCustomCategory>({
  name: "",
  transaction_type: "Expense",
});

const transactionTypes = ["Income", "Expense"];

// Group categories by transaction type
const categoriesByType = computed(() => {
  const grouped = new Map<string, CustomCategory[]>();

  categoryStore.categories.forEach((category) => {
    const type = category.transaction_type;
    if (!grouped.has(type)) {
      grouped.set(type, []);
    }
    grouped.get(type)!.push(category);
  });

  return Array.from(grouped.entries()).map(([type, categories]) => ({
    type,
    typeLabel: t(`transactions.types.${type.toLowerCase()}`),
    categories,
  }));
});

const openCreateForm = () => {
  isCreating.value = true;
  editingCategory.value = null;
  newCategory.value = {
    name: "",
    transaction_type: "Expense",
  };
};

const openEditForm = (category: CustomCategory) => {
  isCreating.value = false;
  editingCategory.value = category;
  newCategory.value = {
    name: category.name,
    transaction_type: category.transaction_type,
  };
};

const cancelForm = () => {
  isCreating.value = false;
  editingCategory.value = null;
  newCategory.value = {
    name: "",
    transaction_type: "Expense",
  };
};

const handleCreateCategory = async () => {
  if (!newCategory.value.name || newCategory.value.name.trim() === "") {
    useToastStore().showError(t("categories.name_required"));
    return;
  }

  const success = await categoryStore.createCategory(newCategory.value);
  if (success) {
    cancelForm();
  }
};

const handleUpdateCategory = async () => {
  if (!editingCategory.value) {
    return;
  }

  if (!newCategory.value.name || newCategory.value.name.trim() === "") {
    useToastStore().showError(t("categories.name_required"));
    return;
  }

  const updateData: UpdateCustomCategory = {
    name: newCategory.value.name,
    transaction_type: newCategory.value.transaction_type,
  };

  const success = await categoryStore.updateCategory(
    editingCategory.value.id,
    updateData,
  );
  if (success) {
    cancelForm();
  }
};

const handleDeleteCategory = async (id: string) => {
  await categoryStore.deleteCategory(id);
};

onMounted(async () => {
  if (categoryStore.categories.length === 0) {
    await categoryStore.loadCategories();
  }
});
</script>

<template>
  <GlassDialog
    :visible="visible"
    @update:visible="emit('update:visible', $event)"
    :header="t('categories.manage_categories')"
    class="w-full max-w-3xl"
  >
    <div class="space-y-4">
      <!-- Create/Edit Form -->
      <div v-if="isCreating || editingCategory" class="p-4 bg-white/10 rounded-lg border border-white/20">
        <h4 class="text-white font-medium mb-4">
          {{ editingCategory ? t("categories.edit_category") : t("categories.create_category") }}
        </h4>
        <div class="space-y-3">
          <div>
            <label class="block mb-2 text-white/90">
              <i class="pi pi-tag mr-2"></i>{{ t("categories.category_name") }}
            </label>
            <GlassInput
              v-model="newCategory.name"
              :placeholder="t('categories.enter_category_name')"
            />
          </div>
          <div>
            <label class="block mb-2 text-white/90">
              <i class="pi pi-folder mr-2"></i>{{ t("transactions.type") }}
            </label>
            <GlassDropdown
              v-model="newCategory.transaction_type"
              :options="transactionTypes"
            />
          </div>
          <div class="flex gap-2 justify-end">
            <Button
              :label="t('wallets.cancel')"
              icon="pi pi-times"
              text
              @click="cancelForm"
              class="text-white! hover:bg-white/10!"
            />
            <GlassButton
              v-if="editingCategory"
              :label="t('settings.save_settings')"
              icon="pi pi-check"
              @click="handleUpdateCategory"
              :loading="categoryStore.loading"
            />
            <GlassButton
              v-else
              :label="t('categories.create_category')"
              icon="pi pi-plus"
              @click="handleCreateCategory"
              :loading="categoryStore.loading"
            />
          </div>
        </div>
      </div>

      <!-- New Category Button -->
      <div v-if="!isCreating && !editingCategory" class="flex justify-end">
        <GlassButton
          :label="t('categories.new_category')"
          icon="pi pi-plus"
          @click="openCreateForm"
        />
      </div>

      <!-- Categories List -->
      <div v-if="categoryStore.loading" class="flex justify-center py-8 text-white">
        <ProgressSpinner
          style="width: 50px; height: 50px"
          strokeWidth="4"
          fill="transparent"
          animationDuration="1s"
        />
      </div>

      <div v-else-if="categoryStore.categories.length === 0 && !isCreating && !editingCategory" class="text-center py-8 text-white">
        <i class="pi pi-folder-open text-4xl mb-3 opacity-50"></i>
        <p class="opacity-80">{{ t("categories.no_categories") }}</p>
      </div>

      <div v-else class="space-y-4">
        <div
          v-for="group in categoriesByType"
          :key="group.type"
          class="space-y-2"
        >
          <h4 class="text-white/90 font-medium">{{ group.typeLabel }}</h4>
          <div class="space-y-2">
            <Card
              v-for="category in group.categories"
              :key="category.id"
              class="backdrop-blur-xl! bg-white/10! border! border-white/30!"
            >
              <template #content>
                <div class="flex justify-between items-center text-white">
                  <span>{{ category.name }}</span>
                  <div class="flex gap-2">
                    <Button
                      icon="pi pi-pencil"
                      text
                      rounded
                      @click="openEditForm(category)"
                      class="text-white! hover:bg-white/20!"
                      size="small"
                    />
                    <Button
                      icon="pi pi-trash"
                      text
                      rounded
                      severity="danger"
                      @click="handleDeleteCategory(category.id)"
                      class="text-red-300! hover:bg-red-500/20!"
                      size="small"
                      :loading="categoryStore.loading"
                    />
                  </div>
                </div>
              </template>
            </Card>
          </div>
        </div>
      </div>
    </div>
  </GlassDialog>
</template>
