export interface IClaimMeta {
  private?: boolean
  claims?: string[]
  any?: boolean
}

export interface IRouteMeta {
  RouteName: string;
  Operation: string;
  authentication?: boolean;
  module?:string;
}
