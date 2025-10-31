import axios from "axios";

import type { HealthResponse } from "./types";

// Email service axios instance
// Dev: http://localhost:8001
// Prod: /api/email (proxied by nginx)
const emailApi = axios.create({
  baseURL: import.meta.env.PROD ? "/api/email" : "http://localhost:8001",
});

// Auth service axios instance for health endpoint
const authHealthApi = axios.create({
  baseURL: import.meta.env.PROD ? "/api/auth" : "http://localhost:8000",
});

// Settings service axios instance for health endpoint
const settingsHealthApi = axios.create({
  baseURL: import.meta.env.PROD ? "/api/settings" : "http://localhost:8002",
});

export const versionService = {
  /**
   * Get version information from auth service
   */
  async getAuthVersion(): Promise<string> {
    try {
      const response = await authHealthApi.get<HealthResponse>("/health");
      return response.data.version;
    } catch (error) {
      console.error("Failed to fetch auth version:", error);
      return "unknown";
    }
  },

  /**
   * Get version information from settings service
   */
  async getSettingsVersion(): Promise<string> {
    try {
      const response = await settingsHealthApi.get<HealthResponse>("/health");
      return response.data.version;
    } catch (error) {
      console.error("Failed to fetch settings version:", error);
      return "unknown";
    }
  },

  /**
   * Get version information from email service
   */
  async getEmailVersion(): Promise<string> {
    try {
      const response = await emailApi.get<HealthResponse>("/health");
      return response.data.version;
    } catch (error) {
      console.error("Failed to fetch email version:", error);
      return "unknown";
    }
  },

  /**
   * Get frontend version from package.json
   */
  getFrontendVersion(): string {
    return import.meta.env.VITE_APP_VERSION || "unknown";
  },
};
