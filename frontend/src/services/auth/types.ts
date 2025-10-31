export interface LoginResponse {
  token: string;
}

export interface TranslationKeyMessage {
  translation_key: string;
}

export type RegisterResponse = TranslationKeyMessage;
export type ForgotPasswordResponse = TranslationKeyMessage;
export type ChangePasswordResponse = TranslationKeyMessage;
export type LogoutResponse = TranslationKeyMessage;
export type ActivateResponse = TranslationKeyMessage;
