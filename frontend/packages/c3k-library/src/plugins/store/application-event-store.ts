import { defineStore } from "pinia";
import { useSystemStore } from './system-store';

export type OperationType = 'add' | 'edit' | 'delete' | 'refresh' | 'nothing' | string;

export interface OperationState {
    Open: boolean;
    Title: string;
    OperationType: OperationType;
    store: any
};

export const useApplicationEventStore = defineStore('application', {
    state: () => ({
        Open: false,
        Title: '',
        OperationType: 'nothing',
        ...useSystemStore().$state
    }),
    getters: {
        operation: (state) => state.Title.toLocaleLowerCase().split(' ')[0]
    },
    actions: {
        setDrawerEvent(action: Partial<OperationState>) {
            this.Open = action.Open ?? this.Open;
            this.Title = action.Title ?? this.Title;
            this.OperationType = action.OperationType ?? this.OperationType;
        },
        setDefaultOperation(operationType: OperationType, title: string) {
            this.OperationType = operationType;
            this.Title = title;
            this.Open = true;
        },
        resetOperation() {
            this.OperationType = 'nothing';
            this.Title = '';
            this.Open = false;
        },
        setLoading(action: boolean) {
            this.isLoading = action;
        }
    },
});