import { client } from './client';

type ApiResult<TData> = Promise<{
  data?: TData;
  error?: unknown;
  response: Response;
}>;

type ApiMethod = <TData>(path: string, init: Record<string, unknown>) => ApiResult<TData>;

interface ApiClient {
  del: ApiMethod;
  get: ApiMethod;
  post: ApiMethod;
  put: ApiMethod;
}

const { apiClient, handleApiResponse } = client;

const api: ApiClient = {
  del: apiClient.DELETE,
  get: apiClient.GET,
  post: apiClient.POST,
  put: apiClient.PUT,
};

export { api, handleApiResponse, type ApiClient, type ApiResult };
