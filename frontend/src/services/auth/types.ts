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
export type VerifyResponse = TranslationKeyMessage;

export interface PasskeyRegisterStartResponse {
  user_id: string;
  creation_options: any;
}

export interface PasskeyLoginStartResponse {
  request_options: any;
}
