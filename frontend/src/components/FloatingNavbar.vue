<script setup lang="ts">
import { computed } from "vue";
import { useRoute, useRouter } from "vue-router";

const styles = {
    unselectedButtonPt: {
        root: "lg:!min-w-[100px] lg:!max-w-[100px] !max-w-[80px] !min-w-[80-px] !text-xs !rounded-2xl !bg-transparent !border-transparent hover:!bg-gradient-to-br from-white/60 to-blue-200/40",
    },
    selectedButtonPt: {
        root: "lg:!min-w-[100px] lg:!max-w-[100px] !max-w-[80px] !min-w-[80-px] !text-xs !rounded-2xl !text-blue-600 !bg-transparent !bg-gradient-to-br from-white/60 to-blue-200/40 !backdrop-blur-md !border-transparent !shadow-[0_2px_8px_rgba(0,0,0,0.15)] !ring-1 !ring-black/5",
    },
    unselectedRoundButtonPt: {
        root: "!max-w-[75px] !text-xs !bg-transparent !border-transparent hover:!bg-gradient-to-br from-white/60 to-blue-200/40",
    },
    selectedRoundButtonPt: {
        root: "!max-w-[75px] !text-xs !bg-transparent !text-blue-600 !bg-gradient-to-br from-white/60 to-blue-200/40 !backdrop-blur-md !border-transparent !shadow-[0_2px_8px_rgba(0,0,0,0.15)] !ring-1 !ring-black/5",
    },
};

enum NavbarPages {
    DASHBOARD = "dashboard",
    WALLETS = "wallets",
    ADD = "add",
    ANALYTICS = "analytics",
    SETTINGS = "settings",
    OTHER = "other",
};

const route = useRoute();
const router = useRouter();

const currentPage = computed(() => {
    switch (route.name) {
        case "home":
            return NavbarPages.DASHBOARD;
        case "wallets":
            return NavbarPages.WALLETS;
        case "add":
            return NavbarPages.ADD;
        case "analytics":
            return NavbarPages.ANALYTICS;
        case "settings":
            return NavbarPages.SETTINGS;
        default:
            return NavbarPages.OTHER;
    }
});
</script>

<template>
    <div className="fixed bottom-6 left-1/2 transform -translate-x-1/2 z-50">
        <nav className="backdrop-blur-2xl bg-white/10 border border-white/20 rounded-2xl shadow-2xl px-2 py-3">
            <div className="absolute inset-0 bg-gradient-to-r from-white/20 via-white/10 to-white/20 rounded-2xl" />
            <div className="relative flex items-center gap-1">
                <Button @click="router.push('/')" icon="pi pi-home" iconPos="top" label="Dashboard"
                    :pt="currentPage === NavbarPages.DASHBOARD ? styles.selectedButtonPt : styles.unselectedButtonPt" />
                <Button @click="router.push('/wallets')" icon="pi pi-wallet" iconPos="top" label="Wallets"
                    :pt="currentPage === NavbarPages.WALLETS ? styles.selectedButtonPt : styles.unselectedButtonPt" />
                <Button @click="router.push('/add')" class="min-w-[60px] min-h-[60px]" icon="pi pi-plus" iconPos="top"
                    :pt="currentPage === NavbarPages.ADD ? styles.selectedRoundButtonPt : styles.unselectedRoundButtonPt"
                    rounded />
                <Button @click="router.push('/analytics')" icon="pi pi-chart-bar" iconPos="top" label="Analytics"
                    :pt="currentPage === NavbarPages.ANALYTICS ? styles.selectedButtonPt : styles.unselectedButtonPt" />
                <Button @click="router.push('/settings')" icon="pi pi-cog" iconPos="top" label="Settings"
                    :pt="currentPage === NavbarPages.SETTINGS ? styles.selectedButtonPt : styles.unselectedButtonPt" />
            </div>
        </nav>
    </div>
</template>
