import { defineStore } from 'pinia';
import { initGlobalState } from 'qiankun';

export const useGlobalStateStore = defineStore('globalState', {
  state: () => ({
    globalState: {
      user: null,
      userModules: [],
      sideBarMenu: [],
      toggleSidebar: false,
    },
  }),
  actions: {
    initializeQiankunState(initialState: any) {
      // Initialize Qiankun's global state
      const actions = initGlobalState(initialState);

      // Listen for state changes
      actions.onGlobalStateChange((state: any) => {
        console.log('[Qiankun Global State Updated]', state);
        this.globalState = state; // Sync to Pinia store
      }, true);

      // Save setGlobalState for later use
      this.setGlobalState = actions.setGlobalState;
    },

    setGlobalState(_state: any) {
      console.warn('setGlobalState not initialized!');
    },

    updateGlobalState(newState: Partial<typeof this.globalState>) {
      // Merge the new state with the existing state
      this.globalState = { ...this.globalState, ...newState };
      // Propagate changes to Qiankun's global state
      this.setGlobalState(this.globalState);
    },
  },
});
