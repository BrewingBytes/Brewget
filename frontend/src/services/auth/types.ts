export interface LoginResponse {
    token: string;
};

export interface RegisterResponse {
    message: string;
};

export interface ForgotPasswordResponse {
    message: string;
};

export interface ChangePasswordResponse {
    message: string;
};

export interface LogoutResponse {
    message: string;
};

export interface ActivateResponse {
    message: string;
};

export interface PasskeyRegisterStartResponse {
    challenge: any;
    state: string;
};

export interface PasskeyRegisterFinishResponse {
    message: string;
};

export interface PasskeyAuthStartResponse {
    challenge: any;
    state: string;
};

export interface PasskeyAuthFinishResponse {
    token: string;
};
