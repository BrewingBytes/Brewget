import type { ActivateResponse, ChangePasswordResponse, ForgotPasswordResponse, LoginResponse, LogoutResponse, RegisterResponse } from "./types";
import type { ErrorResponse, ServerResponse } from "@/services/types";
import type { AxiosError } from "axios";

import axios from "@/services/api";
import { useAuthStore } from "@/stores/auth";

const URL_PATH = import.meta.env.PROD ? "/auth" : "";

async function activate(values: { id: string }): Promise<ServerResponse<ActivateResponse>> {
    try {
        return await axios.get(`${URL_PATH}/activate/${values.id}`);
    } catch (error) {
        return (error as AxiosError).response as ErrorResponse;
    }
}

async function login(values: { username: string, password: string }): Promise<ServerResponse<LoginResponse>> {
    try {
        return await axios.post(`${URL_PATH}/login`, values);
    } catch (error) {
        return (error as AxiosError).response as ErrorResponse;
    }
}

async function register(values: { email: string, username: string, password: string }): Promise<ServerResponse<RegisterResponse>> {
    try {
        return await axios.post(`${URL_PATH}/register`, values);
    } catch (error) {
        return (error as AxiosError).response as ErrorResponse;
    }
}

async function forgotPassword(values: { email: string }): Promise<ServerResponse<ForgotPasswordResponse>> {
    try {
        return await axios.post(`${URL_PATH}/forgot-password`, values);
    } catch (error) {
        return (error as AxiosError).response as ErrorResponse;
    }
}

async function changePassword(values: { id: string, password: string }): Promise<ServerResponse<ChangePasswordResponse>> {
    try {
        return await axios.post(`${URL_PATH}/change-password`, values);
    } catch (error) {
        return (error as AxiosError).response as ErrorResponse;
    }
}

async function logout(): Promise<ServerResponse<LogoutResponse>> {
    try {
        return await axios.get(`${URL_PATH}/logout`, {
            headers: {
                Authorization: useAuthStore().bearerToken,
            },
        });
    } catch (error) {
        return (error as AxiosError).response as ErrorResponse;
    }
}

export const authService = { activate, changePassword, forgotPassword, login, logout, register };
