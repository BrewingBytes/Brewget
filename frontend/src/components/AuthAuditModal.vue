<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";

import type { AuthenticationAuditLog } from "@/services/auth/types";

import { authService } from "@/services/auth";

const props = defineProps<{
  visible: boolean;
}>();

const emit = defineEmits<{
  (e: "update:visible", value: boolean): void;
}>();

const { t } = useI18n();
const auditLogs = ref<AuthenticationAuditLog[]>([]);
const loading = ref(true);

onMounted(async () => {
  await loadAuditLogs();
});

async function loadAuditLogs() {
  loading.value = true;
  try {
    const response = await authService.auditList();
    if (response.status === 200 && response.data) {
      auditLogs.value = response.data;
    }
  } catch (error) {
    console.error("Failed to load audit logs:", error);
  } finally {
    loading.value = false;
  }
}

function handleClose() {
  emit("update:visible", false);
}

function formatDate(dateString: string): string {
  const date = new Date(dateString);
  return new Intl.DateTimeFormat("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  }).format(date);
}

function getAuthMethodLabel(method: string): string {
  return t(`auth_audit.methods.${method.toLowerCase()}`);
}

function getStatusLabel(success: boolean): string {
  return success ? t("auth_audit.status.success") : t("auth_audit.status.failed");
}

function getStatusClass(success: boolean): string {
  return success ? "text-green-400" : "text-red-400";
}

function getMethodIcon(method: string): string {
  switch (method) {
    case "password":
      return "pi-lock";
    case "passkey":
      return "pi-key";
    case "otp":
      return "pi-shield";
    default:
      return "pi-question-circle";
  }
}
</script>

<template>
  <Dialog :visible="props.visible" modal :closable="true" @update:visible="handleClose" :header="t('auth_audit.title')"
    :style="{ width: '90vw', maxWidth: '1000px' }" :contentStyle="{ maxHeight: '70vh', overflow: 'auto' }" :pt="{
      root: {
        class: 'backdrop-blur-2xl! bg-transparent! border! border-white/20! shadow-2xl!',
      },
      header: {
        class: 'bg-transparent! border-b! border-white/20! text-white!',
      },
      content: {
        class: 'bg-transparent! text-white!',
      }
    }" pt:mask:class="backdrop-blur-xs! bg-transparent!">
    <template #closebutton>
      <button type="button" class="p-2 hover:cursor-pointer hover:bg-white/10 rounded-full" @click="handleClose">
        <i class="pi pi-times text-xl"></i>
      </button>
    </template>

    <div v-if="loading" class="flex justify-center py-8">
      <ProgressSpinner style="width: 50px; height: 50px" strokeWidth="4" />
    </div>

    <div v-else>
      <div v-if="auditLogs.length === 0" class="text-center py-8">
        <p class="text-white/70">{{ t("auth_audit.no_logs") }}</p>
      </div>

      <div v-else class="space-y-3">
        <div v-for="log in auditLogs" :key="log.id"
          class="p-4 rounded-lg bg-white/10 border border-white/20 hover:bg-white/15 transition-colors">
          <div class="flex items-start justify-between">
            <div class="flex items-start gap-3 flex-1">
              <i :class="`pi ${getMethodIcon(log.auth_method)} text-2xl text-white/80`"></i>
              <div class="flex-1">
                <div class="flex items-center gap-2 mb-1">
                  <span class="font-semibold text-white">{{ getAuthMethodLabel(log.auth_method) }}</span>
                  <span :class="`font-medium ${getStatusClass(log.success)}`">
                    {{ getStatusLabel(log.success) }}
                  </span>
                </div>
                <div class="text-sm text-white/70 space-y-1">
                  <div class="flex items-center gap-2">
                    <i class="pi pi-calendar text-xs"></i>
                    <span>{{ formatDate(log.attempted_at) }}</span>
                  </div>
                  <div v-if="log.ip_address" class="flex items-center gap-2">
                    <i class="pi pi-map-marker text-xs"></i>
                    <span>{{ log.ip_address }}</span>
                  </div>
                  <div v-if="log.user_agent" class="flex items-center gap-2">
                    <i class="pi pi-desktop text-xs"></i>
                    <span class="truncate max-w-md">{{ log.user_agent }}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </Dialog>
</template>
