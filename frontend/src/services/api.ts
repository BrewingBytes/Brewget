import axios from "axios";

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

// Default export for backwards compatibility
export default authApi;
