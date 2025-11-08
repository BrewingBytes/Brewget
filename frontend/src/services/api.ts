import axios, { type AxiosInstance } from "axios";

import router from "@/router";

// Auth service axios instance
// Dev: http://localhost:8000
// Prod: /api/auth (proxied by nginx)
export const authApi = axios.create({
    baseURL: import.meta.env.PROD ? "/api/auth" : "http://localhost:8000",
});

// Settings service axios instance
// Dev: http://localhost:8001
// Prod: /api/settings (proxied by nginx)
export const settingsApi = axios.create({
    baseURL: import.meta.env.PROD ? "/api/settings" : "http://localhost:8001",
});

// Wallet service axios instance  
// Dev: http://localhost:8002 (currently proxied to settings-service)
// Prod: /api/wallets (proxied by nginx)
// Note: Wallet functionality is temporarily in settings-service, will be extracted to separate service
export const walletApi = axios.create({
    baseURL: import.meta.env.PROD ? "/api/wallets" : "http://localhost:8001/wallets",
});

// Add response interceptor to handle token expiration globally
const setupInterceptors = (apiInstance: AxiosInstance) => {
    apiInstance.interceptors.response.use(
        (response) => response,
        async (error) => {
            // Check if error is 401 and has TOKEN_EXPIRED translation key
            if (
                error.response?.status === 401 &&
                (error.response?.data?.translation_key === "TOKEN_EXPIRED" || error.response?.data?.translation_key === "TOKEN_INVALID")
            ) {
                // Import dynamically to avoid circular dependency
                const { useAuthStore } = await import("@/stores/auth");
                const { useToastStore, ToastSeverity } = await import("@/stores/toast");

                const authStore = useAuthStore();
                const toastStore = useToastStore();

                // Show error message
                toastStore.showTranslationKey(error.response?.data?.translation_key, ToastSeverity.ERROR);

                // Use logout method to ensure proper cleanup
                authStore.token = "";
                router.push("/login");
            }

            return Promise.reject(error);
        },
    );
};

// Setup interceptors for all API instances
setupInterceptors(authApi);
setupInterceptors(settingsApi);
setupInterceptors(walletApi);

// Default export for backwards compatibility
export default authApi;
