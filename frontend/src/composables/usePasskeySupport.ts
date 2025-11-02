import { onMounted, ref } from "vue";
import { checkPasskeySupport, type PasskeySupport } from "@/services/webauthn";

/**
 * Composable to detect and track passkey support in the browser
 * @returns Reactive passkey support status
 */
export function usePasskeySupport() {
  const passkeySupport = ref<PasskeySupport>({
    available: false,
    platformAuthenticator: false,
  });

  const isLoading = ref(true);

  onMounted(async () => {
    try {
      passkeySupport.value = await checkPasskeySupport();
    } catch (error) {
      console.error("Failed to check passkey support:", error);
    } finally {
      isLoading.value = false;
    }
  });

  return {
    passkeySupport,
    isLoading,
  };
}
