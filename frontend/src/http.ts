import type {
  AxiosError,
  AxiosInstance,
  AxiosRequestConfig,
  AxiosResponse,
} from 'axios'
import axios from 'axios'
// @ts-expect-error "Idk the problem with my linter but print is not defined"
import { identity } from 'ramda'

import { push } from 'notivue'
import { config } from './lib/config'
import type { components } from '@/types'
import ProblemFactory from '@/utils/problemFactory'
import { getAccessToken, getRefreshToken } from '@/iam'

type definitions = components['schemas']

async function getAxios(
  authenticated: boolean = true,
  use_refresh_token: boolean = false,
): Promise<AxiosInstance> {
  const token = authenticated
    ? use_refresh_token
      ? getRefreshToken().value
      : getAccessToken().value
    : null
  const headers
    = token !== null
      ? {
          Authorization: `Bearer ${token}`,
        }
      : {}

  const client = axios.create({
    baseURL: config.API_ENDPOINT,
    timeout: config.API_TIMEOUT,
    headers,
    withCredentials: !!config.FRONTEND_DEV_MODE, // false in dev mode, true in staging/production mode
  })

  client.interceptors.response.use(identity, (error: AxiosError) => {
    // Any status codes that falls outside the range of 2xx cause this function to trigger

    // convert timeouts axios's error to Problem
    if (isAxiosError(error) && String(error.message).includes('timeout of')) {
      return Promise.reject(
        new ProblemFactory(0, 'TimeoutExceeded', error.message, error.message),
      )
    }

    return Promise.reject(error)
  })

  return client
}

export default {
  get<T = any, R = AxiosResponse<T>>(
    url: string,
    config?: AxiosRequestConfig,
  ): Promise<R> {
    return getAxios().then(axios => axios.get(url, config))
  },
  delete<T = any, R = AxiosResponse<T>>(
    url: string,
    config?: AxiosRequestConfig,
  ): Promise<R> {
    return getAxios().then(axios => axios.delete(url, config))
  },
  head<T = any, R = AxiosResponse<T>>(
    url: string,
    config?: AxiosRequestConfig,
  ): Promise<R> {
    return getAxios().then(axios => axios.head(url, config))
  },
  options<T = any, R = AxiosResponse<T>>(
    url: string,
    config?: AxiosRequestConfig,
  ): Promise<R> {
    return getAxios().then(axios => axios.options(url, config))
  },
  post<T = any, R = AxiosResponse<T>>(
    url: string,
    data?: any,
    config?: AxiosRequestConfig,
  ): Promise<R> {
    return getAxios().then(axios => axios.post(url, data, config))
  },
  put<T = any, R = AxiosResponse<T>>(
    url: string,
    data?: any,
    config?: AxiosRequestConfig,
  ): Promise<R> {
    return getAxios().then(axios => axios.put(url, data, config))
  },
  patch<T = any, R = AxiosResponse<T>>(
    url: string,
    data?: any,
    config?: AxiosRequestConfig,
  ): Promise<R> {
    return getAxios().then(axios => axios.patch(url, data, config))
  },

  unauthenticated: {
    get<T = any, R = AxiosResponse<T>>(
      url: string,
      config?: AxiosRequestConfig,
    ): Promise<R> {
      return getAxios(false).then(axios => axios.get(url, config))
    },
    post<T = any, R = AxiosResponse<T>>(
      url: string,
      data?: any,
      config?: AxiosRequestConfig,
    ): Promise<R> {
      return getAxios(false).then(axios => axios.post(url, data, config))
    },
  },

  withRefreshToken: {
    post<T = any, R = AxiosResponse<T>>(
      url: string,
      data?: any,
      config?: AxiosRequestConfig,
    ): Promise<R> {
      return getAxios(true, true).then(axios =>
        axios.post(url, data, config),
      )
    },
  },
}

// Global types
export type UUID = string

export type Problem = definitions['Problem']

export function handleError(err: AxiosError<AxiosResponse<Problem>>): Problem {
  if (err.response?.data && typeof err.response.data === 'object') {
    const problem = err.response.data as unknown as Problem
    if (
      typeof problem.detail === 'string'
      && typeof problem.status === 'number'
      && typeof problem.id === 'string'
      && typeof problem.title === 'string'
    ) {
      return problem
    }
  }
  return {
    id: 'unknown',
    title: 'Unknown Error',
    status: 500,
    detail: `An unknown error occurred: ${err.message}`,
  }
}

export function isAxiosError(err: unknown): err is AxiosError {
  const e = err as AxiosError
  return e !== null && typeof e.isAxiosError === 'boolean' && e.isAxiosError
}

export function displayError(err: AxiosError<AxiosResponse<Problem>>) {
  const problem = handleError(err)
  const options = {
    title: problem.title,
    message: problem.detail,
    duration: 5000,
  }
  if (problem.status >= 500) {
    push.error(options)
  }
  else {
    push.warning(options)
  }
}

export function displayProblem(problem: Problem) {
  const options = {
    title: problem.title,
    message: problem.detail,
    duration: 5000,
  }
  if (problem.status >= 500) {
    push.error(options)
  }
  else {
    push.warning(options)
  }
}
