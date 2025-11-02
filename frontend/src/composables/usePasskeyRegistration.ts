import { ref } from "vue";

import { authService } from "@/services/auth";
import type { ServerResponse } from "@/services/types";
import { ServerStatus } from "@/services/types";
import { credentialToJSON, registerPasskey } from "@/services/webauthn";
import { ToastSeverity, useToastStore } from "@/stores/toast";

/**
 * Composable for handling passkey registration flow
 * Provides reusable functions for both initial registration and adding passkeys to existing accounts
 */
export function usePasskeyRegistration() {
  const isRegistering = ref(false);
  const toast = useToastStore();

  /**
   * Register a new passkey during account registration
   * @param params Registration parameters
   * @returns Promise<boolean> indicating success
   */
  async function registerPasskeyForNewAccount(params: {
    username: string;
    email: string;
    captchaToken: string;
    deviceName?: string;
  }): Promise<boolean> {
    isRegistering.value = true;
    try {
      // Start passkey registration
      const startResponse = await authService.passkeyRegisterStart({
        username: params.username,
        email: params.email,
        captchaToken: params.captchaToken,
      });

      if (startResponse.status !== ServerStatus.NO_ERROR) {
        handleError(startResponse);
        return false;
      }

      // Create passkey with the user's authenticator
      const credential = await registerPasskey(
        startResponse.data.creation_options,
      );
      const credentialJSON = credentialToJSON(credential);

      // Complete passkey registration
      const finishResponse = await authService.passkeyRegisterFinish({
        user_id: startResponse.data.user_id,
        credential: credentialJSON,
        device_name: params.deviceName,
      });

      if (finishResponse.status !== ServerStatus.NO_ERROR) {
        handleError(finishResponse);
        return false;
      }

      toast.showTranslationKey(
        finishResponse.data.translation_key,
        ToastSeverity.SUCCESS,
      );

      return true;
    } catch (error) {
      console.error("Passkey registration error:", error);
      toast.showTranslationKey(
        "PASSKEY_REGISTRATION_FAILED",
        ToastSeverity.ERROR,
      );
      return false;
    } finally {
      isRegistering.value = false;
    }
  }

  /**
   * Add a new passkey to an existing authenticated account
   * @param deviceName Optional device name for the passkey
   * @returns Promise<boolean> indicating success
   */
  async function addPasskeyToExistingAccount(
    deviceName?: string,
  ): Promise<boolean> {
    isRegistering.value = true;
    try {
      // Start passkey addition
      const startResponse = await authService.passkeyAddStart();

      if (startResponse.status !== ServerStatus.NO_ERROR) {
        handleError(startResponse);
        return false;
      }

      // Register passkey with browser
      const credential = await registerPasskey(
        startResponse.data.creation_options,
      );
      const credentialJSON = credentialToJSON(credential);

      // Complete passkey addition
      const finishResponse = await authService.passkeyAddFinish({
        user_id: startResponse.data.user_id,
        credential: credentialJSON,
        device_name: deviceName,
      });

      if (finishResponse.status !== ServerStatus.NO_ERROR) {
        handleError(finishResponse);
        return false;
      }

      toast.showTranslationKey(
        finishResponse.data.translation_key,
        ToastSeverity.SUCCESS,
      );

      return true;
    } catch (error) {
      console.error("Failed to add passkey:", error);
      toast.showTranslationKey(
        "PASSKEY_REGISTRATION_FAILED",
        ToastSeverity.ERROR,
      );
      return false;
    } finally {
      isRegistering.value = false;
    }
  }

  /**
   * Handle error responses from the server
   * @param response Error response from the server
   */
  function handleError(response: ServerResponse<unknown>) {
    const errorKey =
      (response.data as { translation_key?: string })?.translation_key ||
      "SOMETHING_WENT_WRONG";
    toast.showTranslationKey(errorKey, ToastSeverity.ERROR);
  }

  return {
    isRegistering,
    registerPasskeyForNewAccount,
    addPasskeyToExistingAccount,
  };
}
