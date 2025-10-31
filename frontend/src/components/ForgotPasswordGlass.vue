<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";

import { useAuthStore } from "@/stores/auth";

const props = defineProps(["changePasswordId"]);
const { t } = useI18n();

const password = ref("");
function buttonAction() {
  useAuthStore().changePassword({
    id: props.changePasswordId as string,
    password: password.value,
  });
}
</script>

<template>
  <div
    class="px-8 md:px-12 lg:px-20 py-12 flex flex-col items-center gap-12 w-full backdrop-blur-2xl rounded-2xl bg-white/10 border border-white/10 max-w-sm"
  >
    <div class="flex flex-col items-center gap-4 w-full">
      <div class="flex flex-col gap-2 w-full">
        <div class="text-center text-3xl font-medium text-white leading-tight">
          BrewGet
        </div>
      </div>
    </div>
    <div class="flex flex-col items-center gap-8 w-full">
      <div class="flex flex-col gap-6 w-full">
        <IconField>
          <InputIcon class="pi pi-lock text-white/70!" />
          <InputText
            v-model="password"
            type="password"
            class="appearance-none! border! border-white/10! w-full! outline-0! bg-white/10! text-white! placeholder:text-white/70! rounded-3xl! shadow-sm!"
            :placeholder="t('auth.placeholders.password')"
          />
        </IconField>
      </div>
      <Button
        @click="buttonAction"
        :label="t('auth.change_password.button')"
        class="w-full! rounded-3xl! bg-surface-950! border! border-surface-950! text-white! hover:bg-surface-950/80!"
      />
    </div>
  </div>
</template>
