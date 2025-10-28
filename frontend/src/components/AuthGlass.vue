<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";

import { useAuthStore } from "@/stores/auth";

const { t } = useI18n();

enum ShownPage {
    Login,
    Register,
    ForgotPassword,
}

const shownPage = ref(ShownPage.Login);
const isLogin = computed(() => shownPage.value === ShownPage.Login);
const isRegister = computed(() => shownPage.value === ShownPage.Register);
const isForgotPassword = computed(() => shownPage.value === ShownPage.ForgotPassword);

const username = ref("");
const password = ref("");
const email = ref("");

const texts = computed(() => {
    switch (shownPage.value) {
        case ShownPage.Login:
            return {
                spanText: t("auth.dontHaveAccount"),
                loginRegisterText: t("auth.signUp"),
                buttonText: t("auth.signIn"),
            };
        case ShownPage.Register:
            return {
                spanText: t("auth.alreadyHaveAccount"),
                loginRegisterText: t("auth.signIn"),
                buttonText: t("auth.register"),
            };
        case ShownPage.ForgotPassword:
        default:
            return {
                spanText: t("auth.forgotPassword"),
                loginRegisterText: "",
                buttonText: t("auth.sendEmail"),
            };
    }
});

function switchLoginRegister() {
    if (shownPage.value === ShownPage.Login) {
        return shownPage.value = ShownPage.Register;
    }

    return shownPage.value = ShownPage.Login;
}

function resetToLogin() {
    email.value = "";
    username.value = "";
    password.value = "";

    shownPage.value = ShownPage.Login;
}

async function buttonAction() {
    if (isLogin.value) {
        await useAuthStore().login({
            username: username.value,
            password: password.value,
        });
    } else if (isRegister.value) {
        if (
            await useAuthStore().register({
                email: email.value,
                username: username.value,
                password: password.value,
            })) {
            resetToLogin();
        }
    } else if (isForgotPassword.value) {
        if (
            await useAuthStore().forgotPassword({
                email: email.value,
            })) {
            resetToLogin();
        }
    }
}
</script>

<template>
    <div
        class="px-8 md:px-12 lg:px-20 py-12 flex flex-col items-center gap-12 w-full backdrop-blur-2xl rounded-2xl bg-white/10 border border-white/10 max-w-sm">
        <div class="flex flex-col items-center gap-4 w-full">
            <div class="flex flex-col gap-2 w-full">
                <div class="text-center text-3xl font-medium text-white leading-tight">BrewGet</div>
                <div class="text-center">
                    <span class="text-white/80">{{ texts.spanText }} </span>
                    <a class="text-white/80 cursor-pointer hover:text-white/90 underline"
                        @click="switchLoginRegister">{{
                            texts.loginRegisterText }}</a>
                </div>
            </div>
        </div>
        <div class="flex flex-col items-center gap-8 w-full">
            <div class="flex flex-col gap-6 w-full">
                <IconField v-if="isRegister || isForgotPassword">
                    <InputIcon class="pi pi-at text-white/70!" />
                    <InputText v-model="email" type="text"
                        class="appearance-none! border! border-white/10! w-full! outline-0! bg-white/10! text-white! placeholder:text-white/70! rounded-3xl! shadow-sm!"
                        :placeholder="t('auth.email')" />
                </IconField>
                <IconField v-if="!isForgotPassword">
                    <InputIcon class="pi pi-user text-white/70!" />
                    <InputText v-model="username" type="text"
                        class="appearance-none! border! border-white/10! w-full! outline-0! bg-white/10! text-white! placeholder:text-white/70! rounded-3xl! shadow-sm!"
                        :placeholder="t('auth.username')" />
                </IconField>
                <IconField v-if="!isForgotPassword">
                    <InputIcon class="pi pi-lock text-white/70!" />
                    <InputText v-model="password" type="password"
                        class="appearance-none! border! border-white/10! w-full! outline-0! bg-white/10! text-white! placeholder:text-white/70! rounded-3xl! shadow-sm!"
                        :placeholder="t('auth.password')" />
                </IconField>
            </div>
            <Button @click="buttonAction" :label="texts.buttonText"
                class="w-full! rounded-3xl! bg-surface-950! border! border-surface-950! text-white! hover:bg-surface-950/80!" />
        </div>
        <a v-if="isLogin" @click="shownPage = ShownPage.ForgotPassword"
            class="text-white/80 cursor-pointer hover:text-white/90">{{ t("auth.forgotPassword") }}</a>
        <a v-if="isForgotPassword" @click="shownPage = ShownPage.Login"
            class="text-white/80 cursor-pointer hover:text-white/90">{{ t("auth.goBack") }}</a>
    </div>
</template>
