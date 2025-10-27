import type {
  Settings,
  UpdateSettings,
} from "./types";
import type { ErrorResponse, ServerResponse } from "@/services/types";
import type { AxiosError } from "axios";

import axios from "@/services/api";
import { useAuthStore } from "@/stores/auth";

const URL_PATH = import.meta.env.PROD ? "/api/settings/user" : "/user";

async function getSettings(): Promise<ServerResponse<Settings>> {
  try {
    return await axios.get(`${URL_PATH}`, {
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
    return await axios.post(`${URL_PATH}`, settings, {
      headers: {
        Authorization: useAuthStore().bearerToken,
      },
    });
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

export const settingsService = { getSettings, updateSettings };
