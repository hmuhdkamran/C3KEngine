<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { UsersService } from "@/services/role/users-service";
import StatisticsCard from '@/components/StatisticsCard.vue';
import UserGrowthChart from '@/components/UserGrowthChart.vue';
import RolesDistributionChart from '@/components/RolesDistributionChart.vue';
import RecentActivities from '@/components/RecentActivities.vue';
import UserTable from '@/components/UserTable.vue';

const users = ref([]);
const recentActivities = ref([]);

const loadUsers = async () => {
    try {
        const response = await new UsersService().GetAll();
        if (response?.data) {
            users.value = response.data;
            console.log("Users loaded:", users.value);
            recentActivities.value = response.data.map(user => ({
                activity: `${user.Username} ${user.Status ? 'activated' : 'deactivated'} their account`,
                timestamp: new Date().toLocaleString(),
            }));
        }
    } catch (error) {
        console.error("Failed to load users:", error);
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

onMounted(() => loadUsers());
</script>
<template>
    <div class="p-8 space-y-6 bg-gray-100 min-h-screen">
        <header class="flex items-center justify-between pb-4 border-b border-gray-300">
            <h2 class="text-2xl font-bold text-gray-800">User Management</h2>
            <router-link to="role/user">
                <button class="px-3 py-1 rounded-md btn-primary">View Users</button>
            </router-link>
        </header>

        <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-4 bg-gray-100">
            <StatisticsCard title="Total Users" :count="totalUsers">
                <p class="text-sm text-gray-500">Active: {{ activeUsers }} | Deactive: {{ deactiveUsers }}</p>
            </StatisticsCard>

            <StatisticsCard title="Roles Overview">
                <ul class="mt-2 grid grid-cols-2 gap-y-1">
                    <li v-for="(count, role) in roles" :key="role" class="flex items-center text-sm text-gray-800">
                        {{ role }}: {{ count }}
                    </li>
                </ul>
            </StatisticsCard>

            <StatisticsCard title="New Users This Week" :count="newUsersThisWeek" />

            <StatisticsCard title="Manage Roles">
                <router-link to="/role/manage">
                    <button class="mt-6 px-3 py-2 rounded-md w-full btn-primary">Manage Roles</button>
                </router-link>
            </StatisticsCard>
        </div>

        <section class="grid grid-cols-4 gap-4">
            <div class="col-span-3">
                <UserGrowthChart />
            </div>
            <div class="col-span-1">
                <RolesDistributionChart :roles="roles" />
            </div>
        </section>

        <section class="grid grid-cols-1 gap-6 md:grid-cols-2">
            <RecentActivities :activities="recentActivities" />
            <UserTable :users="users" />
        </section>
    </div>
</template>