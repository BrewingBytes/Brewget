import type { ActivateResponse, ChangePasswordResponse, ForgotPasswordResponse, LoginResponse, LogoutResponse, PasskeyLoginStartResponse, PasskeyRegisterStartResponse, RegisterResponse, VerifyResponse } from "./types";
import type { ErrorResponse, ServerResponse } from "@/services/types";
import type { AxiosError } from "axios";

import { authApi } from "@/services/api";
import { useAuthStore } from "@/stores/auth";

async function activate(values: { id: string }): Promise<ServerResponse<ActivateResponse>> {
    try {
        return await authApi.get(`/activate/${values.id}`);
    } catch (error) {
        return (error as AxiosError).response as ErrorResponse;
    }
}

async function login(values: { username: string, password: string, captchaToken: string }): Promise<ServerResponse<LoginResponse>> {
    try {
        return await authApi.post("/login", values);
    } catch (error) {
        return (error as AxiosError).response as ErrorResponse;
    }
}

async function register(values: { email: string, username: string, password: string, captchaToken: string }): Promise<ServerResponse<RegisterResponse>> {
    try {
        return await authApi.post("/register", values);
    } catch (error) {
        return (error as AxiosError).response as ErrorResponse;
    }
}

async function forgotPassword(values: { email: string, captchaToken: string }): Promise<ServerResponse<ForgotPasswordResponse>> {
    try {
        return await authApi.post("/forgot-password", values);
    } catch (error) {
        return (error as AxiosError).response as ErrorResponse;
    }
}

async function changePassword(values: { id: string, password: string }): Promise<ServerResponse<ChangePasswordResponse>> {
    try {
        return await authApi.post("/change-password", values);
    } catch (error) {
        return (error as AxiosError).response as ErrorResponse;
    }
}

async function logout(): Promise<ServerResponse<LogoutResponse>> {
    try {
        return await authApi.get("/logout", {
            headers: {
                Authorization: useAuthStore().bearerToken,
            },
        });
    } catch (error) {
        return (error as AxiosError).response as ErrorResponse;
    }
}

async function verify(): Promise<ServerResponse<VerifyResponse>> {
    try {
        return await authApi.get("/verify", {
            headers: {
                Authorization: useAuthStore().bearerToken,
            },
        });
    } catch (error) {
        const axiosError = error as AxiosError;
        if (axiosError.response) {
            return axiosError.response as ErrorResponse;
        }
        // Fallback if response is not available
        throw error;
    }
}

async function passkeyRegisterStart(values: {
  username: string;
  email: string;
  captchaToken: string;
}): Promise<ServerResponse<PasskeyRegisterStartResponse>> {
  try {
    return await authApi.post("/passkey/register/options", values);
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

async function passkeyRegisterFinish(values: {
  user_id: string;
  credential: Record<string, unknown>;
  device_name?: string;
}): Promise<ServerResponse<RegisterResponse>> {
  try {
    return await authApi.post("/passkey/register/complete", values);
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

async function passkeyLoginStart(values: {
  username: string;
  captchaToken: string;
}): Promise<ServerResponse<PasskeyLoginStartResponse>> {
  try {
    return await authApi.post("/passkey/login/options", values);
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

async function passkeyLoginFinish(values: {
  username: string;
  credential: Record<string, unknown>;
}): Promise<ServerResponse<LoginResponse>> {
  try {
    return await authApi.post("/passkey/login/complete", values);
  } catch (error) {
    return (error as AxiosError).response as ErrorResponse;
  }
}

export const authService = { activate, changePassword, forgotPassword, login, logout, passkeyLoginFinish, passkeyLoginStart, passkeyRegisterFinish, passkeyRegisterStart, register, verify };
