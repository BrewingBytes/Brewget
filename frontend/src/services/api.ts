import axios from "axios";
import { useI18n } from "vue-i18n";

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

// Add interceptor to include Accept-Language header
const addLanguageHeader = (config: any) => {
    try {
        const { locale } = useI18n();
        if (locale && locale.value) {
            config.headers = config.headers || {};
            config.headers['Accept-Language'] = locale.value;
        }
    } catch (e) {
        // If i18n is not available, don't add the header
    }
    return config;
};

authApi.interceptors.request.use(addLanguageHeader);
settingsApi.interceptors.request.use(addLanguageHeader);

// Default export for backwards compatibility
export default authApi;
