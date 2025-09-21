<script setup lang="ts">
import { computed } from "vue";
import { useRoute } from "vue-router";

import AuthGlass from "@/components/AuthGlass.vue";
import ForgotPasswordGlass from "@/components/ForgotPasswordGlass.vue";
import { useAuthStore } from "@/stores/auth";


const route = useRoute();

const isAuth = computed(() => route.name === "login");

if (route.name === "activate") {
    useAuthStore().activate({
        id: route.params.id as string,
    });
}
</script>

<template>
    <div class="pa-0 flex items-center justify-center backdrop-blur-3xl bg-cover! bg-center! bg-no-repeat!"
        style="background-image: url('https://fqjltiegiezfetthbags.supabase.co/storage/v1/object/public/block.images/blocks/signin/signin-glass.jpg')">
        <AuthGlass v-if="isAuth" />
        <ForgotPasswordGlass v-else :changePasswordId="route.params.id" />
    </div>
</template>
