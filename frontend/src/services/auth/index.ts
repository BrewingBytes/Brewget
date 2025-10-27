import type { ActivateResponse, ChangePasswordResponse, ForgotPasswordResponse, LoginResponse, LogoutResponse, RegisterResponse } from "./types";
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

async function login(values: { username: string, password: string }): Promise<ServerResponse<LoginResponse>> {
    try {
        return await authApi.post(`/login`, values);
    } catch (error) {
        return (error as AxiosError).response as ErrorResponse;
    }
}

async function register(values: { email: string, username: string, password: string }): Promise<ServerResponse<RegisterResponse>> {
    try {
        return await authApi.post(`/register`, values);
    } catch (error) {
        return (error as AxiosError).response as ErrorResponse;
    }
}

async function forgotPassword(values: { email: string }): Promise<ServerResponse<ForgotPasswordResponse>> {
    try {
        return await authApi.post(`/forgot-password`, values);
    } catch (error) {
        return (error as AxiosError).response as ErrorResponse;
    }
}

async function changePassword(values: { id: string, password: string }): Promise<ServerResponse<ChangePasswordResponse>> {
    try {
        return await authApi.post(`/change-password`, values);
    } catch (error) {
        return (error as AxiosError).response as ErrorResponse;
    }
}

async function logout(): Promise<ServerResponse<LogoutResponse>> {
    try {
        return await authApi.get(`/logout`, {
            headers: {
                Authorization: useAuthStore().bearerToken,
            },
        });
    } catch (error) {
        return (error as AxiosError).response as ErrorResponse;
    }
}

export const authService = { activate, changePassword, forgotPassword, login, logout, register };
