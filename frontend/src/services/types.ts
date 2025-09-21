import type { AxiosResponse } from "axios";

export type Error = {
    message: string
};

export type SuccessResponse<T> = AxiosResponse<T> & { status: ServerStatus.NO_ERROR };
export type ErrorResponse = AxiosResponse<Error> & { status: Exclude<number, ServerStatus.NO_ERROR> };
export type ServerResponse<T> = SuccessResponse<T> | ErrorResponse;

export enum ServerStatus {
    NO_ERROR = 200,
};
