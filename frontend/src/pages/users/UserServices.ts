import type { AxiosError, AxiosResponse } from "axios";
import type { Problem } from "@/http.ts";
import http, { handleError } from "@/http.ts";

export async function deleteUser(): Promise<void> {
  return http.delete("/user").then(
    (res: AxiosResponse<void>) => res.data,
    (err: AxiosError<AxiosResponse<Problem>>) =>
      Promise.reject(handleError(err)),
  );
}

export async function changePassword(new_password: string): Promise<void> {
  return http
    .post("/auth/password", {
      new_password,
    })
    .then(
      (res: AxiosResponse<void>) => res.data,
      (err: AxiosError<AxiosResponse<Problem>>) =>
        Promise.reject(handleError(err)),
    );
}

export async function verifyEmail(token: string): Promise<void> {
  return http.unauthenticated.post(`/auth/verify-email`, { token }).then(
    (res: AxiosResponse<void>) => res.data,
    (err: AxiosError<AxiosResponse<Problem>>) =>
      Promise.reject(handleError(err)),
  );
}

export async function resendVerificationEmail(token: string): Promise<void> {
  return http.unauthenticated
    .post(`/auth/resend-verification-email`, { token })
    .then(
      (res: AxiosResponse<void>) => res.data,
      (err: AxiosError<AxiosResponse<Problem>>) =>
        Promise.reject(handleError(err)),
    );
}

export async function beginResetPassword(email: string): Promise<void> {
  return http.unauthenticated
    .post(`/auth/begin-reset-password`, { email })
    .then(
      (res: AxiosResponse<void>) => res.data,
      (err: AxiosError<AxiosResponse<Problem>>) =>
        Promise.reject(handleError(err)),
    );
}

export async function resetPassword(
  token: string,
  new_password: string,
): Promise<void> {
  return http.unauthenticated
    .post(`/auth/reset-password`, { token, new_password })
    .then(
      (res: AxiosResponse<void>) => res.data,
      (err: AxiosError<AxiosResponse<Problem>>) =>
        Promise.reject(handleError(err)),
    );
}
