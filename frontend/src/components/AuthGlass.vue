<script setup lang="ts">
import { computed, ref } from "vue";

import { useAuthStore } from "@/stores/auth";
import { isWebAuthnSupported } from "@/utils/webauthn";

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
const usePasskey = ref(false);

const supportsPasskeys = isWebAuthnSupported();

const texts = computed(() => {
    switch (shownPage.value) {
        case ShownPage.Login:
            return {
                spanText: "Don't have an account?",
                loginRegisterText: "Sign up",
                buttonText: usePasskey.value ? "Sign In with Passkey" : "Sign In",
            };
        case ShownPage.Register:
            return {
                spanText: "Already have an account?",
                loginRegisterText: "Sign in",
                buttonText: usePasskey.value ? "Register with Passkey" : "Register",
            };
        case ShownPage.ForgotPassword:
        default:
            return {
                spanText: "Forgot your password?",
                loginRegisterText: "",
                buttonText: "Send me an email",
            };
    }
});

function switchLoginRegister() {
    usePasskey.value = false;
    if (shownPage.value === ShownPage.Login) {
        return shownPage.value = ShownPage.Register;
    }

    return shownPage.value = ShownPage.Login;
}

function resetToLogin() {
    email.value = "";
    username.value = "";
    password.value = "";
    usePasskey.value = false;

    shownPage.value = ShownPage.Login;
}

async function buttonAction() {
    if (isLogin.value) {
        if (usePasskey.value) {
            await useAuthStore().loginWithPasskey({
                username: username.value,
            });
        } else {
            await useAuthStore().login({
                username: username.value,
                password: password.value,
            });
        }
    } else if (isRegister.value) {
        let success = false;
        if (usePasskey.value) {
            success = await useAuthStore().registerWithPasskey({
                email: email.value,
                username: username.value,
            });
        } else {
            success = await useAuthStore().register({
                email: email.value,
                username: username.value,
                password: password.value,
            });
        }
        if (success) {
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
                        placeholder="Email" />
                </IconField>
                <IconField v-if="!isForgotPassword">
                    <InputIcon class="pi pi-user text-white/70!" />
                    <InputText v-model="username" type="text"
                        class="appearance-none! border! border-white/10! w-full! outline-0! bg-white/10! text-white! placeholder:text-white/70! rounded-3xl! shadow-sm!"
                        placeholder="Username" />
                </IconField>
                <IconField v-if="!isForgotPassword && !usePasskey">
                    <InputIcon class="pi pi-lock text-white/70!" />
                    <InputText v-model="password" type="password"
                        class="appearance-none! border! border-white/10! w-full! outline-0! bg-white/10! text-white! placeholder:text-white/70! rounded-3xl! shadow-sm!"
                        placeholder="Password" />
                </IconField>
                
                <!-- Passkey Toggle -->
                <div v-if="supportsPasskeys && !isForgotPassword" class="flex items-center gap-2">
                    <Checkbox v-model="usePasskey" :binary="true" inputId="usePasskey"
                        class="border-white/10!" />
                    <label for="usePasskey" class="text-white/80 text-sm cursor-pointer">
                        {{ isLogin ? "Sign in with passkey" : "Register with passkey only" }}
                    </label>
                </div>
            </div>
            <Button @click="buttonAction" :label="texts.buttonText"
                class="w-full! rounded-3xl! bg-surface-950! border! border-surface-950! text-white! hover:bg-surface-950/80!" />
        </div>
        <a v-if="isLogin" @click="shownPage = ShownPage.ForgotPassword"
            class="text-white/80 cursor-pointer hover:text-white/90">Forgot
            Password?</a>
        <a v-if="isForgotPassword" @click="shownPage = ShownPage.Login"
            class="text-white/80 cursor-pointer hover:text-white/90">Go back</a>
    </div>
</template>
