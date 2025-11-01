<script setup lang="ts">
import { onMounted, ref } from "vue";

import { versionService } from "@/services/version";

interface ChangeEntry {
  version: string;
  date: string;
  changes: {
    added: string[];
    changed: string[];
    fixed: string[];
  };
}

interface ServiceChangelog {
  service: string;
  entries: ChangeEntry[];
}

interface VersionInfo {
  service: string;
  version: string;
  entries: ChangeEntry[];
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

    // Fetch pre-built changelogs JSON
    const changelogsData = await fetchChangelogsJson();

    // Map service names to versions
    const versionMap: Record<string, string> = {
      "Frontend": frontendVersion,
      "Auth Service": authVersion,
      "Settings Service": settingsVersion,
      "Email Service": emailVersion,
    };

    // Build version info with changelogs
    versions.value = changelogsData.map((changelog) => ({
      service: changelog.service,
      version: versionMap[changelog.service] || "unknown",
      entries: changelog.entries,
    }));
  } catch (error) {
    console.error("Failed to load versions:", error);
  } finally {
    loading.value = false;
  }
}

async function fetchChangelogsJson(): Promise<ServiceChangelog[]> {
  try {
    const response = await fetch("/changelogs.json");
    if (!response.ok) {
      console.error("Failed to fetch changelogs.json");
      return [];
    }
    return await response.json();
  } catch (error) {
    console.error("Failed to fetch changelogs JSON:", error);
    return [];
  }
}

function handleClose() {
  emit("update:visible", false);
}
</script>

<template>
  <Dialog :visible="props.visible" modal :closable="true" @update:visible="handleClose" header="Changelogs"
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
      <TabView v-model:activeIndex="activeTab" :pt="{
        nav: {
          class: 'bg-transparent! border-b! border-white/20!',
        },
        inkbar: {
          class: 'bg-black!',
        },
        panelContainer: {
          class: 'bg-transparent!',
        }
      }" :pt:tabpanel="{
        headerAction: {
          class: 'text-white! hover:text-white!'
        }
      }">
        <TabPanel v-for="(versionInfo, index) in versions" :key="index" :value="index"
          :header="`${versionInfo.service} (v${versionInfo.version})`">
          <Accordion v-if="versionInfo.entries.length > 0" :multiple="true" :activeIndex="[0]">
            <AccordionTab v-for="(entry, entryIndex) in versionInfo.entries" :key="entryIndex" :pt="{
              header: {
                class: 'bg-white/20! text-white/90!',
              },
              content: {
                class: 'bg-white/10! text-white/90!',
              },
              headerIcon: {
                class: 'text-white/90!',
              }
            }">
              <template #header>
                <span class="font-semibold">Version {{ entry.version }} - {{ entry.date }}</span>
              </template>
              <div class="space-y-3">
                <div v-if="entry.changes.added.length > 0">
                  <p class="font-bold mb-2">Added</p>
                  <ul class="list-disc list-inside space-y-1 ml-2">
                    <li v-for="(change, idx) in entry.changes.added" :key="idx" class="text-sm">{{ change }}</li>
                  </ul>
                </div>
                <div v-if="entry.changes.changed.length > 0">
                  <p class="font-bold mb-2">Changed</p>
                  <ul class="list-disc list-inside space-y-1 ml-2">
                    <li v-for="(change, idx) in entry.changes.changed" :key="idx" class="text-sm">{{ change }}</li>
                  </ul>
                </div>
                <div v-if="entry.changes.fixed.length > 0">
                  <p class="font-bold mb-2">Fixed</p>
                  <ul class="list-disc list-inside space-y-1 ml-2">
                    <li v-for="(change, idx) in entry.changes.fixed" :key="idx" class="text-sm">{{ change }}</li>
                  </ul>
                </div>
              </div>
            </AccordionTab>
          </Accordion>
          <div v-else class="text-center py-4">
            <p class="text-white/70">No changelog available</p>
          </div>
        </TabPanel>
      </TabView>
    </div>
  </Dialog>
</template>
