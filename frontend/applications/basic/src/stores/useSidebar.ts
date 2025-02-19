import { ref } from 'vue';

const isSidebarOpen = ref(true);

export function useSidebar() {
    return {
        isSidebarOpen,
        toggleSidebar: () => {
            isSidebarOpen.value = !isSidebarOpen.value;
        },
    };
}