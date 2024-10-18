import { ref } from 'vue';

// Extend the Notification interface to include the title
interface Notification {
  id: number;
  title: string;
  message: string;
  type: 'success' | 'error' | 'warning' | 'info';
  position: 'top-right' | 'top-left' | 'bottom-right' | 'bottom-left';
}

const notifications = ref<Notification[]>([]);

export function useNotification() {
  // Add title to the parameters of addNotification
  const addNotification = (
    message: string, 
    type: 'success' | 'error' | 'warning' | 'info', 
    position: 'top-right' | 'top-left' | 'bottom-right' | 'bottom-left', 
    title: string,
    duration = 3000
  ) => {
    const id = Date.now();
    // Add title when pushing notification
    notifications.value.push({ id, title, message, type, position });

    setTimeout(() => {
      removeNotification(id);
    }, duration);
  };

  const removeNotification = (id: number) => {
    notifications.value = notifications.value.filter(notification => notification.id !== id);
  };

  return {
    notifications,
    addNotification,
    removeNotification,
  };
}
