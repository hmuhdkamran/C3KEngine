import type { AxiosError, AxiosResponse } from 'axios';
import type { IPayload } from '@/types/axios';

export class PayloadMapper {
  private fromError<T>(error: Error): IPayload<T> {
    return {
      data: {} as T,
      result: 'error',
      description: error.name,
    };
  }

  private fromAxiosError<T>(error: AxiosError): IPayload<T> {
    let data: T = {} as T;

    if (error.response && isAxiosResponse(error.response)) {
      data = this.fromAxiosResponse<T>(error.response).data;
    }

    return {
      data,
      result: 'error',
      description: `Code: ${error.code}. ${error.name} = ${error.message}`,
    };
  }

  private fromAxiosResponse<T>(response: AxiosResponse): IPayload<T> {
    if (isPayload<T>(response.data)) {
      return response.data;
    }

    return {
      data: response.data as T, // Use type assertion here instead of 'any'
      result: 'success',
      description: '',
    };
  }

  public fromObject<T>(obj: unknown): IPayload<T | null> | null {
    if (isAxiosError(obj)) {
      return this.fromAxiosError<T>(obj);
    }

    if (obj instanceof Error) {
      return this.fromError<T>(obj);
    }

    if (isAxiosResponse(obj)) {
      return this.fromAxiosResponse<T>(obj);
    }

    return null;
  }
}

function isAxiosResponse(obj: unknown): obj is AxiosResponse {
  return (
    typeof obj === 'object' &&
    obj !== null &&
    'data' in obj &&
    'config' in obj &&
    'status' in obj &&
    'statusText' in obj &&
    'headers' in obj
  );
}

function isAxiosError(obj: unknown): obj is AxiosError {
  return (
    typeof obj === 'object' &&
    obj instanceof Error &&
    'config' in obj
  );
}

function isPayload<T>(obj: unknown): obj is IPayload<T> {
  return (
    typeof obj === 'object' &&
    obj !== null &&
    'data' in obj &&
    'message' in obj
  );
}
