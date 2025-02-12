import { acceptHMRUpdate } from 'pinia'
import { createGenericPiniaStore } from 'c3-library';
import { IStatus } from '~/models/setup/setup';

export const useSetupStatusStore = createGenericPiniaStore<IStatus, 'StatusId'>('SetupStatus', 'auth/setup/status', 'StatusId');

if (import.meta.hot)
    import.meta.hot.accept(acceptHMRUpdate(useSetupStatusStore, import.meta.hot))