import { defineStore } from 'pinia';
import { UsersService } from '@/services';

export interface IApplicationEvent {
  ToggleDrawer: boolean;
  data: [];
}

const usersService = new UsersService();

export const useApplicationEventStore = defineStore('application', {
  state: (): IApplicationEvent => ({
    ToggleDrawer: false,
    data: []
  }),
  actions: {
    // refreshData() {
    //   console.log('Refreshing data...');
    //     const fetchedData =  usersService.GetAll();
    //     this.data = fetchedData || [];
    //     console.log('Data refreshed successfully:', this.data);
    // },
    exportData() {
      console.log('Exporting data...');
      if (this.data.length === 0) {
        console.warn('No data to export.');
        return;
      }
      const csvContent = this.data
        .map(row => Object.values(row).join(','))
        .join('\n');
      const blob = new Blob([csvContent], { type: 'text/csv' });
      const link = document.createElement('a');
      link.href = URL.createObjectURL(blob);
      link.download = 'exported_data.csv';
      link.click();
    }
  }
});
