<script setup lang="ts">
import { ref } from "vue";

const stats = [
  { id: 1, label: "Total Users", value: "1,234", icon: "👤", bgColor: "bg-gray-200" },
  { id: 2, label: "Active Projects", value: "87", icon: "📁", bgColor: "bg-gray-200" },
  { id: 3, label: "Support Tickets", value: "23", icon: "📨", bgColor: "bg-gray-200" },
  { id: 4, label: "Revenue", value: "$12,345", icon: "💰", bgColor: "bg-gray-200" },
];

const recentActivities = [
  { id: 1, user: "John Doe", activity: "updated the project 'Website Redesign'", time: "2 hours ago" },
  { id: 2, user: "Mary Smith", activity: "resolved ticket #456", time: "4 hours ago" },
  { id: 3, user: "James Johnson", activity: "added a new user", time: "6 hours ago" },
];

const tasks = [
  { id: 1, task: "Finish UI Design", status: "In Progress" },
  { id: 2, task: "Review Team Feedback", status: "Pending" },
  { id: 3, task: "Submit Report", status: "Completed" },
];

const notifications = [
  { id: 1, message: "Server maintenance scheduled at 12:00 AM.", time: "1 hour ago" },
  { id: 2, message: "New support ticket received.", time: "3 hours ago" },
  { id: 3, message: "User registration request pending approval.", time: "5 hours ago" },
];
</script>

<template>
  <main class="flex-1 min-h-screen">
    <div class="container mx-auto grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mt-4">
      <div v-for="stat in stats" :key="stat.id"
        class="p-4 rounded-lg shadow-md flex items-center hover:shadow-lg transition-shadow" :class="stat.bgColor">
        <div class="text-4xl mr-4">{{ stat.icon }}</div>
        <div>
          <p class="text-lg font-semibold">{{ stat.label }}</p>
          <p class="text-2xl font-bold">{{ stat.value }}</p>
        </div>
      </div>
    </div>

    <section class="container mx-auto mt-8">
      <h2 class="text-lg font-bold text-gray-700 mb-4">Tasks Overview</h2>
      <div class="bg-white rounded-lg shadow-md p-4">
        <ul>
          <li v-for="task in tasks" :key="task.id"
            class="flex justify-between items-center py-3 border-b border-gray-300 last:border-b-0">
            <span>{{ task.task }}</span>
            <span class="px-2 py-1 rounded text-sm" :class="{
              'bg-yellow-100 text-yellow-700': task.status === 'Pending',
              'bg-blue-100 text-blue-700': task.status === 'In Progress',
              'bg-green-100 text-green-700': task.status === 'Completed'
            }">
              {{ task.status }}
            </span>
          </li>
        </ul>
      </div>
    </section>

    <section class="container mx-auto mt-8">
      <h2 class="text-lg font-bold text-gray-700 mb-4">Recent Activities</h2>
      <div class="bg-white rounded-lg shadow-md p-4">
        <ul>
          <li v-for="activity in recentActivities" :key="activity.id"
            class="flex items-center py-3 border-b border-gray-300 last:border-b-0">
            <div class="w-10 h-10 bg-gray-200 rounded-full flex items-center justify-center mr-4 text-gray-600">
              {{ activity.user.charAt(0) }}
            </div>
            <div>
              <p class="text-sm"><strong>{{ activity.user }}</strong> {{ activity.activity }}</p>
              <p class="text-xs text-gray-500">{{ activity.time }}</p>
            </div>
          </li>
        </ul>
      </div>
    </section>

    <section class="container mx-auto mt-8">
      <h2 class="text-lg font-bold text-gray-700 mb-4">Notifications</h2>
      <div class="bg-white rounded-lg shadow-md p-4">
        <ul>
          <li v-for="notification in notifications" :key="notification.id" class="py-3 border-b border-gray-300 last:border-b-0">
            <p>{{ notification.message }}</p>
            <p class="text-xs text-gray-500">{{ notification.time }}</p>
          </li>
        </ul>
      </div>
    </section>
  </main>
</template>

<style scoped></style>

<route lang="yaml">
meta:
  layout: auth
  redirectIfLoggedIn: true
</route>