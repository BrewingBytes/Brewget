<script setup lang="ts">
import { onMounted, ref } from "vue";

import { versionService } from "@/services/version";

interface VersionInfo {
  service: string;
  version: string;
  changelog: string;
}

const props = defineProps<{
  visible: boolean;
}>();

const emit = defineEmits<{
  (e: "update:visible", value: boolean): void;
}>();

const activeTab = ref(0);
const versions = ref<VersionInfo[]>([]);
const loading = ref(true);

onMounted(async () => {
  await loadVersions();
});

async function loadVersions() {
  loading.value = true;
  try {
    const [frontendVersion, authVersion, settingsVersion, emailVersion] = await Promise.all([
      Promise.resolve(versionService.getFrontendVersion()),
      versionService.getAuthVersion(),
      versionService.getSettingsVersion(),
      versionService.getEmailVersion(),
    ]);

    // Fetch changelog content from public directory
    const [frontendChangelog, authChangelog, settingsChangelog, emailChangelog] = await Promise.all([
      fetchChangelog("frontend"),
      fetchChangelog("auth-service"),
      fetchChangelog("settings-service"),
      fetchChangelog("email-service"),
    ]);

    versions.value = [
      { service: "Frontend", version: frontendVersion, changelog: frontendChangelog },
      { service: "Auth Service", version: authVersion, changelog: authChangelog },
      { service: "Settings Service", version: settingsVersion, changelog: settingsChangelog },
      { service: "Email Service", version: emailVersion, changelog: emailChangelog },
    ];
  } catch (error) {
    console.error("Failed to load versions:", error);
  } finally {
    loading.value = false;
  }
}

async function fetchChangelog(service: string): Promise<string> {
  try {
    const response = await fetch(`/changelogs/${service}-CHANGELOG.md`);
    if (!response.ok) {
      return "Changelog not available";
    }
    return await response.text();
  } catch (error) {
    console.error(`Failed to fetch changelog for ${service}:`, error);
    return "Changelog not available";
  }
}

function handleClose() {
  emit("update:visible", false);
}
</script>

<template>
  <Dialog
    :visible="props.visible"
    modal
    :closable="true"
    @update:visible="handleClose"
    header="Changelogs"
    :style="{ width: '90vw', maxWidth: '1000px' }"
    :contentStyle="{ maxHeight: '70vh', overflow: 'auto' }"
  >
    <div v-if="loading" class="flex justify-center py-8">
      <ProgressSpinner style="width: 50px; height: 50px" strokeWidth="4" />
    </div>
    <div v-else>
      <TabView v-model:activeIndex="activeTab">
        <TabPanel v-for="(version, index) in versions" :key="index" :value="index" :header="`${version.service} (v${version.version})`">
          <div class="prose prose-sm max-w-none">
            <pre class="whitespace-pre-wrap font-sans text-sm">{{ version.changelog }}</pre>
          </div>
        </TabPanel>
      </TabView>
    </div>
  </Dialog>
</template>

<style scoped>
.prose pre {
  background: transparent;
  padding: 0;
  margin: 0;
}
</style>
