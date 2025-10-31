<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import VueTurnstile from "vue-turnstile";

import { useAuthStore } from "@/stores/auth";
import { TURNSTILE_SITE_KEY } from "@/utils/consts";

enum ShownPage {
  Login,
  Register,
  ForgotPassword,
}

const { t } = useI18n();

const shownPage = ref(ShownPage.Login);
const isLogin = computed(() => shownPage.value === ShownPage.Login);
const isRegister = computed(() => shownPage.value === ShownPage.Register);
const isForgotPassword = computed(
  () => shownPage.value === ShownPage.ForgotPassword,
);

const username = ref("");
const password = ref("");
const email = ref("");
const captchaToken = ref("");

const texts = computed(() => {
  switch (shownPage.value) {
    case ShownPage.Login:
      return {
        spanText: t("auth.login.no_account"),
        loginRegisterText: t("auth.login.sign_up"),
        buttonText: t("auth.login.button"),
      };
    case ShownPage.Register:
      return {
        spanText: t("auth.register.have_account"),
        loginRegisterText: t("auth.register.sign_in"),
        buttonText: t("auth.register.button"),
      };
    case ShownPage.ForgotPassword:
    default:
      return {
        spanText: t("auth.forgot_password.title"),
        loginRegisterText: "",
        buttonText: t("auth.forgot_password.button"),
      };
  }
});

function switchLoginRegister() {
  if (shownPage.value === ShownPage.Login) {
    return (shownPage.value = ShownPage.Register);
  }

  return (shownPage.value = ShownPage.Login);
}

function resetToLogin() {
  email.value = "";
  username.value = "";
  password.value = "";
  captchaToken.value = "";

  shownPage.value = ShownPage.Login;
}

function onCaptchaVerify(token: string) {
  captchaToken.value = token;
}

async function buttonAction() {
  if (!captchaToken.value) {
    return;
  }

  if (isLogin.value) {
    await useAuthStore().login({
      username: username.value,
      password: password.value,
      captchaToken: captchaToken.value,
    });
  } else if (isRegister.value) {
    if (
      await useAuthStore().register({
        email: email.value,
        username: username.value,
        password: password.value,
        captchaToken: captchaToken.value,
      })
    ) {
      resetToLogin();
    }
  } else if (isForgotPassword.value) {
    if (
      await useAuthStore().forgotPassword({
        email: email.value,
        captchaToken: captchaToken.value,
      })
    ) {
      resetToLogin();
    }
  }
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
        <div class="text-center">
          <span class="text-white/80">{{ texts.spanText }} </span>
          <a
            class="text-white/80 cursor-pointer hover:text-white/90 underline"
            @click="switchLoginRegister"
            >{{ texts.loginRegisterText }}</a
          >
        </div>
      </div>
    </div>
    <div class="flex flex-col items-center gap-8 w-full">
      <div class="flex flex-col gap-6 w-full">
        <IconField v-if="isRegister || isForgotPassword">
          <InputIcon class="pi pi-at text-white/70!" />
          <InputText
            v-model="email"
            type="text"
            class="appearance-none! border! border-white/10! w-full! outline-0! bg-white/10! text-white! placeholder:text-white/70! rounded-3xl! shadow-sm!"
            :placeholder="t('auth.placeholders.email')"
            @keyup.enter="buttonAction"
          />
        </IconField>
        <IconField v-if="!isForgotPassword">
          <InputIcon class="pi pi-user text-white/70!" />
          <InputText
            v-model="username"
            type="text"
            class="appearance-none! border! border-white/10! w-full! outline-0! bg-white/10! text-white! placeholder:text-white/70! rounded-3xl! shadow-sm!"
            :placeholder="t('auth.placeholders.username')"
            @keyup.enter="buttonAction"
          />
        </IconField>
        <IconField v-if="!isForgotPassword">
          <InputIcon class="pi pi-lock text-white/70!" />
          <InputText
            v-model="password"
            type="password"
            class="appearance-none! border! border-white/10! w-full! outline-0! bg-white/10! text-white! placeholder:text-white/70! rounded-3xl! shadow-sm!"
            :placeholder="t('auth.placeholders.password')"
            @keyup.enter="buttonAction"
          />
        </IconField>
      </div>
      <div class="flex justify-center w-full">
        <VueTurnstile
          v-model="captchaToken"
          :site-key="TURNSTILE_SITE_KEY"
          @verify="onCaptchaVerify"
          theme="dark"
        />
      </div>
      <Button
        @click="buttonAction"
        :label="texts.buttonText"
        class="w-full! rounded-3xl! bg-surface-950! border! border-surface-950! text-white! hover:bg-surface-950/80!"
      />
    </div>
    <a
      v-if="isLogin"
      @click="shownPage = ShownPage.ForgotPassword"
      class="text-white/80 cursor-pointer hover:text-white/90"
      >{{ t("auth.login.forgot_password") }}</a
    >
    <a
      v-if="isForgotPassword"
      @click="shownPage = ShownPage.Login"
      class="text-white/80 cursor-pointer hover:text-white/90"
      >{{ t("auth.forgot_password.go_back") }}</a
    >
  </div>
</template>
