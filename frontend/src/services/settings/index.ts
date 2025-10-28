import type {
  Settings,
  UpdateSettings,
} from "./types";
import type { ErrorResponse, ServerResponse } from "@/services/types";
import type { AxiosError } from "axios";

import { settingsApi } from "@/services/api";
import { useAuthStore } from "@/stores/auth";

async function getSettings(): Promise<ServerResponse<Settings>> {
  try {
    return await settingsApi.get("/user", {
      headers: {
        Authorization: useAuthStore().bearerToken,
      },
    });
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

async function updateSettings(
  settings: UpdateSettings,
): Promise<ServerResponse<Settings>> {
  try {
    return await settingsApi.post("/user", settings, {
      headers: {
        Authorization: useAuthStore().bearerToken,
      },
    });
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

export const settingsService = { getSettings, updateSettings };
