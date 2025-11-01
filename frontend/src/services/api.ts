import axios, { type AxiosInstance } from "axios";

// Auth service axios instance
// Dev: http://localhost:8000
// Prod: /api/auth (proxied by nginx)
export const authApi = axios.create({
    baseURL: import.meta.env.PROD ? "/api/auth" : "http://localhost:8000",
});

// Settings service axios instance
// Dev: http://localhost:8002
// Prod: /api/settings (proxied by nginx)
export const settingsApi = axios.create({
    baseURL: import.meta.env.PROD ? "/api/settings" : "http://localhost:8002",
});

// Add response interceptor to handle token expiration globally
const setupInterceptors = (apiInstance: AxiosInstance) => {
    apiInstance.interceptors.response.use(
        (response) => response,
        async (error) => {
            // Check if error is 401 and has TOKEN_EXPIRED translation key
            if (
                error.response?.status === 401 &&
                error.response?.data?.translation_key === "TOKEN_EXPIRED"
            ) {
                // Import dynamically to avoid circular dependency
                const { useAuthStore } = await import("@/stores/auth");
                const { useToastStore, ToastSeverity } = await import("@/stores/toast");
                
                const authStore = useAuthStore();
                const toastStore = useToastStore();
                
                // Clear token
                authStore.token = "";
                
                // Show error message
                toastStore.showTranslationKey("TOKEN_EXPIRED", ToastSeverity.ERROR);
                
                // Redirect to login
                const { default: router } = await import("@/router");
                router.push("/login");
            }
            
            return Promise.reject(error);
        },
    );
};

// Setup interceptors for both API instances
setupInterceptors(authApi);
setupInterceptors(settingsApi);

// Default export for backwards compatibility
export default authApi;
