import type { AxiosError } from "axios";

import { walletApi } from "@/services/api";
import type { ErrorResponse, ServerResponse } from "@/services/types";
import { useAuthStore } from "@/stores/auth";

export enum Currency {
  USD = "usd",
  EUR = "eur",
  RON = "ron",
}

export enum WalletType {
  GENERAL = "general",
  SAVINGS = "savings",
  BUSINESS = "business",
  PERSONAL = "personal",
}

export interface Wallet {
  id: string;
  user_id: string;
  name: string;
  balance: number;
  currency: Currency;
  wallet_type: WalletType;
  created_at: string;
  updated_at: string;
}

export interface CreateWallet {
  name: string;
  balance?: number;
  currency: Currency;
  wallet_type?: WalletType;
}

export interface UpdateWallet {
  name?: string;
  balance?: number;
  currency?: Currency;
  wallet_type?: WalletType;
}

async function getWallets(): Promise<ServerResponse<Wallet[]>> {
  try {
    return await walletApi.get("/", {
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
    return await walletApi.get(`/${id}`, {
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
    return await walletApi.post("/", wallet, {
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
    return await walletApi.put(`/${id}`, wallet, {
      headers: {
        Authorization: useAuthStore().bearerToken,
      },
    });
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

async function deleteWallet(id: string): Promise<ServerResponse<number>> {
  try {
    return await walletApi.delete(`/${id}`, {
      headers: {
        Authorization: useAuthStore().bearerToken,
      },
    });
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

export const walletService = {
  getWallets,
  getWallet,
  createWallet,
  updateWallet,
  deleteWallet,
};
