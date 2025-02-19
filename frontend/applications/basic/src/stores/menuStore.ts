import { reactive } from "vue";

export const sidebarStore = reactive({
  activeParent: { name: "Home", link: "/dashboard" },

  setActiveParent(parent: { name: string; link: string }) {
    this.activeParent = parent;
  },
});

export default sidebarStore;