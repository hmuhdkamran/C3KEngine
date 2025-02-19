import type { AxiosError, AxiosResponse } from 'axios'
import type { IPayload } from '../models'
import { PayloadMessageTypes } from '../models'

export class PayloadMapper {
  private fromError<T>(o: Error): IPayload<T> {
    return {
      data: {} as T,
      result: PayloadMessageTypes.error,
      description: o.name,
    }
  }

  private fromAxiosError<T>(o: AxiosError): IPayload<T> {
    let data: T = {} as T

    if (o.response && isAxiosResponse(o.response))
      data = this.fromAxiosResponse<T>(o.response).data

    return {
      data,
      result: PayloadMessageTypes.error,
      description: `Code:${o.code}. ${o.name} = ${o.message}`,
    }
  }

  private fromAxiosResponse<T>(o: AxiosResponse): IPayload<T> {
    let value: IPayload<T> | null = null

    if (isPayload<T>(o.data)) { value = o.data }
    else {
      value = {
        data: <any>o.data.data,
        result: <any>o.data.result,
        description: <any>o.data.description,
      }
    }

    return value
  }

  public fromObject<T>(o: any): IPayload<T | null> | null {
    if (isAxiosError(o))
      return this.fromAxiosError<T>(o)

    if (o instanceof Error)
      return this.fromError<T>(o)    

    if (isAxiosResponse(o))
      return this.fromAxiosResponse<T>(o)

    return null
  }
}

function isAxiosResponse(o: any): o is AxiosResponse {
  return (
    o instanceof Object
    && 'data' in o
    && 'config' in o
    && 'status' in o
    && 'statusText' in o
    && 'headers' in o
  )
}

function isAxiosError(o: any): o is AxiosError {
  return o instanceof Object && o instanceof Error && 'config' in o
}

function isPayload<T>(o: any): o is IPayload<T> {
  return o instanceof Object && 'data' in o && 'message' in o
}
