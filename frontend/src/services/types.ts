import type { AxiosResponse } from "axios";

export interface TranslationKeyError {
  translation_key: string;
}

export type SuccessResponse<T> = AxiosResponse<T> & {
  status: ServerStatus.NO_ERROR;
};
export type ErrorResponse = AxiosResponse<TranslationKeyError> & {
  status: Exclude<ServerStatus, ServerStatus.NO_ERROR>;
};
export type ServerResponse<T> = SuccessResponse<T> | ErrorResponse;

export enum ServerStatus {
  NO_ERROR = 200,
  CREATED = 201,
  UNAUTHORIZED = 401,
  BAD_REQUEST = 400,
  UNPROCESSABLE_CONTENT = 422,
}
