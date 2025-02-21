import type { IStatus } from "@/models";
import { createGenericPiniaStore } from "c3-library";
import { acceptHMRUpdate } from "pinia";

export const useSetupStatusStore = createGenericPiniaStore<IStatus, 'StatusId'>('SetupStatus', 'auth/setup/status', 'StatusId');

if (import.meta.hot)
    import.meta.hot.accept(acceptHMRUpdate(useSetupStatusStore, import.meta.hot))