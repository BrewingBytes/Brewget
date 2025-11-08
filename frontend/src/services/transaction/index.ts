import type {
  CreateWallet,
  UpdateWallet,
  Wallet,
} from "./types";
import type { ErrorResponse, ServerResponse } from "@/services/types";
import type { AxiosError } from "axios";

import { transactionApi } from "@/services/api";
import { useAuthStore } from "@/stores/auth";

async function getWallets(): Promise<ServerResponse<Wallet[]>> {
  try {
    return await transactionApi.get("/wallet", {
      headers: {
        Authorization: useAuthStore().bearerToken,
      },
    });
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

async function createWallet(
  wallet: CreateWallet,
): Promise<ServerResponse<Wallet>> {
  try {
    return await transactionApi.post("/wallet", wallet, {
      headers: {
        Authorization: useAuthStore().bearerToken,
      },
    });
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

async function updateWallet(
  id: string,
  wallet: UpdateWallet,
): Promise<ServerResponse<Wallet>> {
  try {
    return await transactionApi.put(`/wallet/${id}`, wallet, {
      headers: {
        Authorization: useAuthStore().bearerToken,
      },
    });
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

async function deleteWallet(id: string): Promise<ServerResponse<void>> {
  try {
    return await transactionApi.delete(`/wallet/${id}`, {
      headers: {
        Authorization: useAuthStore().bearerToken,
      },
    });
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

export const transactionService = {
  getWallets,
  createWallet,
  updateWallet,
  deleteWallet,
};
