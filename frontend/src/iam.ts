import type { App, ComputedRef, Plugin } from "vue";
import { computed, ref } from "vue";

import { differenceInMilliseconds, subMinutes } from "date-fns";
import { push } from "notivue";
import type { UUID } from "@/http";
import http from "@/http";
import type { components } from "@/types";
import router from "@/router/router";
import { routes } from "@/router/routes.ts";

type definitions = components["schemas"];
type LoginResponse = definitions["LoginResponse"];

interface State {
  accessToken: string;
  accessTokenExpiration: Date;
  refreshToken: string;
  refreshTokenExpiration: Date;
  user_id: UUID;
  email: string;
  firstName: string;
  lastName: string;
}

const localStorageKey = "auth";

const state = ref<null | State>(null);
let refreshTimerId: null | number = null;

function readStateFromStorage(): State | null {
  const data = window.localStorage.getItem(localStorageKey);

  if (data !== null) {
    const parsed = JSON.parse(data) as {
      accessToken: string;
      accessTokenExpiration: string;
      refreshToken: string;
      refreshTokenExpiration: string;
      user_id: UUID;
      email: string;
      firstName: string;
      lastName: string;
    } | null;

    if (parsed !== null) {
      const accessTokenExpirationDate = new Date(parsed.accessTokenExpiration);
      const refreshTokenExpirationDate = new Date(
        parsed.refreshTokenExpiration,
      );

      if (refreshTokenExpirationDate <= new Date()) {
        return null;
      } else {
        return {
          ...parsed,
          accessTokenExpiration: accessTokenExpirationDate,
          refreshTokenExpiration: refreshTokenExpirationDate,
        };
      }
    } else {
      return null;
    }
  } else {
    return null;
  }
}

function writeStateToStorage(state: State): void {
  window.localStorage.setItem(localStorageKey, JSON.stringify(state));
}

export function removeStateFromStorage(): void {
  window.localStorage.removeItem(localStorageKey);
}

async function scheduleAutoRefresh(): Promise<void> {
  if (refreshTimerId != null) {
    clearTimeout(refreshTimerId);
  }

  if (state.value) {
    if (state.value.refreshTokenExpiration <= new Date()) {
      state.value = null;
      removeStateFromStorage();
    } else {
      if (state.value.accessTokenExpiration <= new Date()) {
        await refresh().catch(() => {
          state.value = null;
          removeStateFromStorage();
        });
      } else {
        const refreshInMs = differenceInMilliseconds(
          subMinutes(state.value.accessTokenExpiration, 1),
          new Date(),
        );

        refreshTimerId = window.setTimeout(() => {
          refresh().catch(console.error);
        }, refreshInMs);
      }
    }
  } else {
    console.error("Cannot enable token auto refresh is not logged in");
  }
}

export async function login(email: string, password: string): Promise<void> {
  const res = await http.unauthenticated.post<LoginResponse>("/auth/login", {
    email,
    password,
  });
  state.value = {
    accessToken: res.data.access_token,
    accessTokenExpiration: new Date(res.data.access_token_expiration),
    refreshToken: res.data.refresh_token,
    refreshTokenExpiration: new Date(res.data.refresh_token_expiration),
    user_id: res.data.user_id,
    email: res.data.email,
    firstName: res.data.first_name,
    lastName: res.data.last_name,
  };
  if (state.value) {
    writeStateToStorage(state.value);
    await scheduleAutoRefresh();
  }
}

export async function register(
  email: string,
  firstName: string,
  lastName: string,
  password: string,
): Promise<void> {
  return http.unauthenticated.post("/auth/register", {
    email,
    first_name: firstName,
    last_name: lastName,
    password,
  });
}

export async function refresh(): Promise<void> {
  if (state.value) {
    const res =
      await http.withRefreshToken.post<LoginResponse>("/auth/refresh");
    state.value = {
      accessToken: res.data.access_token,
      accessTokenExpiration: new Date(res.data.access_token_expiration),
      refreshToken: res.data.refresh_token,
      refreshTokenExpiration: new Date(res.data.refresh_token_expiration),
      user_id: res.data.user_id,
      email: res.data.email,
      firstName: res.data.first_name,
      lastName: res.data.last_name,
    };
    if (state.value) {
      writeStateToStorage(state.value);
      await scheduleAutoRefresh();
    }
  }
}

export async function logout(): Promise<void> {
  if (state.value) {
    await http.post("/auth/logout");
    if (refreshTimerId !== null) {
      clearTimeout(refreshTimerId);
    }
    state.value = null;
    removeStateFromStorage();
    push.success({
      title: "Success",
      message: "You have been logged out successfully",
      duration: 5000,
    });
    await router.push({ name: routes.Login });
  } else {
    push.error({
      title: "Logout failed",
      message: "Failed to logout, your not logged in",
      duration: 5000,
    });
  }
}

export function getAccessToken(): ComputedRef<null | string> {
  return computed(() => state.value?.accessToken ?? null);
}

export function getRefreshToken(): ComputedRef<null | string> {
  return computed(() => state.value?.refreshToken ?? null);
}

export interface UserInfo {
  user_id: UUID;
  email: string;
  firstName: string;
  lastName: string;
  name: string;
}

export const emptyUserInfo: UserInfo = {
  user_id: "",
  email: "",
  firstName: "",
  lastName: "",
  name: "",
};

export function getUserInfo(): ComputedRef<null | UserInfo> {
  return computed(() => {
    if (state.value) {
      return {
        user_id: state.value.user_id,
        email: state.value.email,
        firstName: state.value.firstName,
        lastName: state.value.lastName,
        name: `${state.value.firstName} ${state.value.lastName}`,
      };
    } else {
      return null;
    }
  });
}

export const AuthPlugin: Plugin = {
  install(_app: App, _options: unknown) {
    const storedState = readStateFromStorage();
    if (storedState !== null) {
      state.value = storedState;
      scheduleAutoRefresh().catch(console.error);
    } else {
      removeStateFromStorage();
    }
    router.beforeEach((to, _from) => {
      // If the route requires authentication and the user is not logged in, redirect to the login page
      if ((to.meta?.requiresAuth ?? true) && state.value === null) {
        return { name: routes.Login };
      }
      // If the route does not require authentication and does not have a redirectIfLoggedIn meta set to false while the user is logged in, then we redirect to the home page
      else if (
        !(to.meta?.requiresAuth ?? true) &&
        (to.meta?.redirectIfLoggedIn ?? true) &&
        state.value !== null
      ) {
        return { name: routes.Home };
      }
      return true;
    });
  },
};
