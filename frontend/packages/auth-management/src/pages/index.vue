<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { Chart, registerables } from 'chart.js';
import { UsersService } from "@/services/role/users-service";

Chart.register(...registerables);

const users = ref([]);
const recentActivities = ref([]);

const loadUsers = async () => {
    const response = await new UsersService().GetAll();
    if (response?.data) {
        users.value = response.data;
        recentActivities.value = response.data.map(user => ({
            activity: `${user.Username} ${user.Status ? 'activated' : 'deactivated'} their account`,
            timestamp: new Date().toLocaleString(),
        }));
    }
};

const totalUsers = computed(() => users.value.length);
const activeUsers = computed(() => users.value.filter(user => user.Status).length);
const deactiveUsers = computed(() => users.value.filter(user => !user.Status).length);
const roles = computed(() => {
    const roleCount: { [key: string]: number } = { Admin: 3, User: 5, Moderator: 2, Guest: 1 };;
    users.value.forEach(user => {
        roleCount[user.role] = (roleCount[user.role] || 0) + 1;
    });
    return roleCount;
});
const newUsersThisWeek = 10;

const userGrowthChartRef = ref(null);
const rolesDistributionChartRef = ref(null);

const setupUserGrowthChart = () => {
    if (userGrowthChartRef.value) {
        new Chart(userGrowthChartRef.value, {
            type: 'line',
            data: {
                labels: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun'],
                datasets: [{
                    label: 'User Growth',
                    data: [10, 20, 15, 25, 30, 40],
                    borderColor: '#4caf50',
                    backgroundColor: 'rgba(76, 175, 80, 0.3)',
                }],
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: { legend: { display: false } },
            },
        });
    }
};

const setupRolesDistributionChart = () => {
    const roleData = Object.values(roles.value);
    if (rolesDistributionChartRef.value && roleData.some(value => value > 0)) {
        new Chart(rolesDistributionChartRef.value, {
            type: 'pie',
            data: {
                labels: Object.keys(roles.value),
                datasets: [{
                    label: 'Roles Distribution',
                    data: roleData,
                    backgroundColor: ['#ff6384', '#36a2eb', '#ffce56'],
                }],
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: { legend: { display: false } },
            },
        });
    }
};

onMounted(() => {
    loadUsers();
    setupUserGrowthChart();
    setupRolesDistributionChart();
});

</script>

<template>
    <div class="p-8 space-y-6 bg-gray-100 min-h-screen">
        <header class="flex items-center justify-between pb-4 border-b border-gray-300">
            <h2 class="text-2xl font-bold text-gray-800">User Management</h2>
            <router-link to="/about">
                <button class="px-3 py-1 rounded-md btn-primary">
                    View Users
                </button>
            </router-link>
        </header>
        <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-4">
            <div class="p-6 bg-white rounded-lg shadow-md">
                <h3 class="text-lg font-semibold text-gray-700">Total Users</h3>
                <p class="mt-2 text-3xl font-bold text-gray-900">{{ totalUsers }}</p>
                <p class="text-sm text-gray-500">Active: {{ activeUsers }} | Deactive: {{ deactiveUsers }}</p>

            </div>

            <div class="p-6 bg-white rounded-lg shadow-md">
                <h3 class="text-lg font-semibold text-gray-700">Roles Overview</h3>
                <ul class="mt-2 grid grid-cols-2 gap-y-1">
                    <li v-for="(count, role) in roles" :key="role" class="text-sm text-gray-800">
                        {{ role }}: {{ count }}
                    </li>
                </ul>
            </div>

            <div class="p-6 bg-white rounded-lg shadow-md">
                <h3 class="text-lg font-semibold text-gray-700">New Users This Week</h3>
                <p class="mt-2 text-3xl font-bold text-gray-900">{{ newUsersThisWeek }}</p>
            </div>

            <div class="p-6 bg-white rounded-lg shadow-md">
                <h3 class="text-lg font-semibold text-gray-700">Manage Roles</h3>
                <router-link to="/role/manage">
                    <button class="mt-6 px-3 py-2 rounded-md w-full btn-primary">
                        Manage Roles
                    </button>
                </router-link>
            </div>
        </div>

        <section class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
            <div class="col-span-2 p-4 bg-white rounded-md shadow-sm">
                <h3 class="text-sm font-semibold text-gray-700">User Growth Over Time</h3>
                <div class="relative h-64">
                    <canvas ref="userGrowthChartRef"></canvas>
                </div>
            </div>
            <div class="p-4 bg-white rounded-md shadow-sm">
                <h3 class="text-sm font-semibold text-gray-700">Roles Distribution</h3>
                <div class="relative h-64">
                    <canvas ref="rolesDistributionChartRef"></canvas>
                </div>
            </div>
        </section>
        <section class="grid grid-cols-1 gap-6 md:grid-cols-2">
            <div class="p-6 bg-white rounded-lg shadow-md">
                <h3 class="text-lg font-semibold text-gray-700">Recent Activity Logs</h3>
                <ul class="mt-4 space-y-2">
                    <li v-for="activity in recentActivities" :key="activity.timestamp" class="text-sm text-gray-500">
                        {{ activity.activity }} - {{ activity.timestamp }}
                    </li>
                </ul>
            </div>

            <div class="p-6 bg-white rounded-lg shadow-md">
                <h3 class="text-lg font-semibold text-gray-700">User List</h3>
                <table class="w-full mt-4 border-collapse">
                    <thead>
                        <tr>
                            <th class="px-4 py-2 text-left text-sm font-semibold text-gray-600">User</th>
                            <th class="px-4 py-2 text-left text-sm font-semibold text-gray-600">Role</th>
                            <th class="px-4 py-2 text-left text-sm font-semibold text-gray-600">Status</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr v-for="user in users" :key="user.UserId">
                            <td class="px-4 py-2 text-gray-800">{{ user.Username }}</td>
                            <td class="px-4 py-2 text-gray-800">{{ user.role }}</td>
                            <td :class="{ 'text-green-600': user.Status, 'text-red-600': !user.Status }"
                                class="px-4 py-2">
                                {{ user.Status ? 'Active' : 'Inactive' }}
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </section>
    </div>
</template>

<style scoped></style>
