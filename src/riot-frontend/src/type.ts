import type { InjectionKey } from 'vue'

export const API_BASE_SYMBOL = Symbol() as InjectionKey<string>
export interface Response {
  status: string
  message: string
}
