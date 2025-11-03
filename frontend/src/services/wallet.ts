// Wallet Types
export interface Wallet {
  id: string;
  user_id: string;
  name: string;
  balance: string;
  currency: string;
  wallet_type: string;
  created_at: string;
  updated_at: string;
}

export interface CreateWallet {
  name: string;
  balance?: number;
  currency?: string;
  wallet_type?: string;
}

export interface UpdateWallet {
  name?: string;
  balance?: number;
  currency?: string;
  wallet_type?: string;
}

// Wallet Service
import type { ErrorResponse, ServerResponse } from "@/services/types";
import type { AxiosError } from "axios";

import { settingsApi } from "@/services/api";
import { useAuthStore } from "@/stores/auth";

async function listWallets(): Promise<ServerResponse<Wallet[]>> {
  try {
    return await settingsApi.get("/wallet", {
      headers: {
        Authorization: useAuthStore().bearerToken,
      },
    });
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

async function getWallet(id: string): Promise<ServerResponse<Wallet>> {
  try {
    return await settingsApi.get(`/wallet/${id}`, {
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
    return await settingsApi.post("/wallet", wallet, {
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
    return await settingsApi.put(`/wallet/${id}`, wallet, {
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
    return await settingsApi.delete(`/wallet/${id}`, {
      headers: {
        Authorization: useAuthStore().bearerToken,
      },
    });
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

export const walletService = {
  listWallets,
  getWallet,
  createWallet,
  updateWallet,
  deleteWallet,
};
