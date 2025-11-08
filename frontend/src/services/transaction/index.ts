import type {
  CreateCustomCategory,
  CreateTransaction,
  CreateWallet,
  CustomCategory,
  Transaction,
  UpdateCustomCategory,
  UpdateTransaction,
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

async function getTransactions(): Promise<ServerResponse<Transaction[]>> {
  try {
    return await transactionApi.get("/transaction", {
      headers: {
        Authorization: useAuthStore().bearerToken,
      },
    });
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

async function getWalletTransactions(
  walletId: string,
): Promise<ServerResponse<Transaction[]>> {
  try {
    return await transactionApi.get(`/transaction/wallet/${walletId}`, {
      headers: {
        Authorization: useAuthStore().bearerToken,
      },
    });
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

async function getTransaction(
  id: string,
): Promise<ServerResponse<Transaction>> {
  try {
    return await transactionApi.get(`/transaction/${id}`, {
      headers: {
        Authorization: useAuthStore().bearerToken,
      },
    });
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

async function createTransaction(
  transaction: CreateTransaction,
): Promise<ServerResponse<Transaction>> {
  try {
    return await transactionApi.post("/transaction", transaction, {
      headers: {
        Authorization: useAuthStore().bearerToken,
      },
    });
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

async function updateTransaction(
  id: string,
  transaction: UpdateTransaction,
): Promise<ServerResponse<Transaction>> {
  try {
    return await transactionApi.put(`/transaction/${id}`, transaction, {
      headers: {
        Authorization: useAuthStore().bearerToken,
      },
    });
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

async function deleteTransaction(id: string): Promise<ServerResponse<void>> {
  try {
    return await transactionApi.delete(`/transaction/${id}`, {
      headers: {
        Authorization: useAuthStore().bearerToken,
      },
    });
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

async function getCategories(): Promise<ServerResponse<CustomCategory[]>> {
  try {
    return await transactionApi.get("/category", {
      headers: {
        Authorization: useAuthStore().bearerToken,
      },
    });
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

async function createCategory(
  category: CreateCustomCategory,
): Promise<ServerResponse<CustomCategory>> {
  try {
    return await transactionApi.post("/category", category, {
      headers: {
        Authorization: useAuthStore().bearerToken,
      },
    });
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

async function updateCategory(
  id: string,
  category: UpdateCustomCategory,
): Promise<ServerResponse<CustomCategory>> {
  try {
    return await transactionApi.put(`/category/${id}`, category, {
      headers: {
        Authorization: useAuthStore().bearerToken,
      },
    });
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

async function deleteCategory(id: string): Promise<ServerResponse<void>> {
  try {
    return await transactionApi.delete(`/category/${id}`, {
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
  getTransactions,
  getWalletTransactions,
  getTransaction,
  createTransaction,
  updateTransaction,
  deleteTransaction,
  getCategories,
  createCategory,
  updateCategory,
  deleteCategory,
};
