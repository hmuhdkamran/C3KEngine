<script setup lang="ts">
import { ref, computed } from 'vue';
import { useRouter } from 'vue-router'; 
import { useTableStore, Card } from 'c3k-library'

const tableStore = useTableStore();

const router = useRouter()
const selectedCategory = ref('All');
const showModulePage = ref(false);
const selectedCardTitle = ref('');
const currentModule = ref('');
const itemsPerPage = ref(20);

const cards = ref([
    // HRMS
    { title: 'Employee Management', description: 'Manage employee data and HR operations', category: 'HRMS', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--account-group]' },
    { title: 'Payroll', description: 'Handle payroll processing and compliance', category: 'HRMS', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--cash-register]' },
    { title: 'Attendance Tracking', description: 'Track employee attendance and leaves', category: 'HRMS', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--calendar-check]' },

    // Retail
    { title: 'Inventory Management', description: 'Manage stock levels and supplier information', category: 'Retail', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--storefront]' },
    { title: 'Point of Sale', description: 'Handle retail transactions and sales data', category: 'Retail', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--cart-outline]' },
    { title: 'Customer Relationship', description: 'Manage customer data and interactions', category: 'Retail', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--account-multiple]' },

    // Production
    { title: 'Production Planning', description: 'Plan and manage production schedules', category: 'Production', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--factory]' },
    { title: 'Quality Control', description: 'Monitor and ensure product quality', category: 'Production', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--check-decagram]' },
    { title: 'Supply Chain Management', description: 'Coordinate supply chain and logistics', category: 'Production', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--truck-delivery]' },

    // Finance
    { title: 'Accounts Payable', description: 'Manage supplier payments and liabilities', category: 'Finance', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--bank-transfer-in]' },
    { title: 'Accounts Receivable', description: 'Track customer payments and receivables', category: 'Finance', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--bank-transfer-out]' },
    { title: 'Budgeting', description: 'Create and manage financial budgets', category: 'Finance', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--finance]' },

    // Marketing
    { title: 'Campaign Management', description: 'Create and track marketing campaigns', category: 'Marketing', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--bullhorn]' },
    { title: 'Lead Management', description: 'Capture and nurture sales leads', category: 'Marketing', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--account-star]' },
    { title: 'Market Analytics', description: 'Analyze market trends and performance', category: 'Marketing', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--chart-line]' },

    // Point of Sales
    { title: 'Transaction Management', description: 'Process and track sales transactions', category: 'Point of Sales', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--credit-card]' },
    { title: 'Sales Reporting', description: 'Generate sales and revenue reports', category: 'Point of Sales', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--file-chart-outline]' },

    // Healthcare
    { title: 'Patient Management', description: 'Manage patient records and history', category: 'Healthcare', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--stethoscope]' },
    { title: 'Appointment Scheduling', description: 'Schedule and manage appointments', category: 'Healthcare', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--calendar-clock]' },
    { title: 'Billing & Insurance', description: 'Manage billing and insurance claims', category: 'Healthcare', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--file-document]' },

    // Education
    { title: 'Student Management', description: 'Manage student data and academic records', category: 'Education', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--school]' },
    { title: 'Course Management', description: 'Create and track courses and schedules', category: 'Education', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--book-open]' },

    // Technology
    { title: 'IT Helpdesk', description: 'Support ticketing and issue tracking', category: 'Technology', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--headset]' },
    { title: 'Project Management', description: 'Plan and execute technology projects', category: 'Technology', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--file-cog]' },

    // Logistics
    { title: 'Fleet Management', description: 'Manage fleet and transportation logistics', category: 'Logistics', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--truck-outline]' },
    { title: 'Warehouse Management', description: 'Oversee warehouse operations and storage', category: 'Logistics', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--warehouse]' },

    // Customer Support
    { title: 'Support Ticketing', description: 'Track and resolve customer support issues', category: 'Customer Support', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--ticket-account]' },
    { title: 'Knowledge Base', description: 'Create and maintain help articles', category: 'Customer Support', status: 'Active', buttonText: 'Explore', iconClass: 'icon-[mdi--library-books]' },
]);

const filteredCards = computed(() => {
    return cards.value.filter((card) => {
        const matchesCategory = selectedCategory.value === 'All' || card.category === selectedCategory.value;
        const matchesSearch = card.title.toLowerCase().includes(tableStore.searchQuery.toLowerCase());
        return matchesCategory && matchesSearch;
    });
});

tableStore.updateTotalRecords(filteredCards.value.length);

const paginatedFilteredCards = computed(() => {
    const start = (tableStore.currentPage - 1) * itemsPerPage.value;
    const end = start + itemsPerPage.value;
    return filteredCards.value.slice(start, end);
});

const selectedCardCategory = computed(() => {
    const selectedCard = cards.value.find(card => card.title === selectedCardTitle.value);
    return selectedCard ? selectedCard.category : 'All';
});


function filterByCategory(category: string) {
    selectedCategory.value = category;
    tableStore.setPage(1);
}

function handleCardClick(cardTitle: string) {
    localStorage.setItem("application", "c3k-auth-management")
    router.replace('/c3k-auth-management');

    // showModulePage.value = true;
    selectedCardTitle.value = cardTitle;
    currentModule.value = cardTitle;
}

</script>

<template>
    <div class="text-blueGray-700 antialiased">
        <div class="bg-white flex flex-col">
            <div class="w-full px-6 py-6 mx-auto">
                <div class="flex justify-end mr-2">
                    <div v-if="!showModulePage" class="flex flex-1">
                        <div class="h-screen bg-white border-r border-gray-300 w-64 mt-8 p-4 hidden sm:block z-20">
                            <div class="text-md font-semibold mb-2">
                                <span class="icon-[ion--folder-sharp] text-violet-600"></span>
                                <span class="text-violet-600">
                                    CATEGORIES
                                </span>
                            </div>
                            <ul class="px-2 text-sm">
                                <li class="mb-1 hover:bg-violet-100 hover:rounded-md py-1 px-3 cursor-pointer"
                                    @click.prevent="filterByCategory('All')">
                                    <a href="#" :class="{ 'text-violet-600': selectedCategory === 'All' }">
                                        ALL
                                    </a>
                                </li>
                                <li class="mb-1 hover:bg-violet-100 hover:rounded-md py-1 px-3 cursor-pointer"
                                    @click.prevent="filterByCategory('HRMS')">
                                    <a href="#" :class="{ 'text-violet-600': selectedCategory === 'HRMS' }">
                                        HRMS
                                    </a>
                                </li>
                                <li class="mb-1 hover:bg-violet-100 hover:rounded-md py-1 px-3 cursor-pointer"
                                    @click.prevent="filterByCategory('Retail')">
                                    <a href="#" :class="{ 'text-violet-600': selectedCategory === 'Retail' }">
                                        Retail
                                    </a>
                                </li>
                                <li class="mb-1 hover:bg-violet-100 hover:rounded-md py-1 px-3 cursor-pointer"
                                    @click.prevent="filterByCategory('Production')">
                                    <a href="#" :class="{ 'text-violet-600': selectedCategory === 'Production' }">
                                        Production
                                    </a>
                                </li>
                                <li class="mb-1 hover:bg-violet-100 hover:rounded-md py-1 px-3 cursor-pointer"
                                    @click.prevent="filterByCategory('Finance')">
                                    <a href="#" :class="{ 'text-violet-600': selectedCategory === 'Finance' }">
                                        Finance
                                    </a>
                                </li>
                                <li class="mb-1 hover:bg-violet-100 hover:rounded-md py-1 px-3 cursor-pointer"
                                    @click.prevent="filterByCategory('Marketing')">
                                    <a href="#" :class="{ 'text-violet-600': selectedCategory === 'Marketing' }">
                                        Marketing
                                    </a>
                                </li>
                                <li class="mb-1 hover:bg-violet-100 hover:rounded-md py-1 px-3 cursor-pointer"
                                    @click.prevent="filterByCategory('Point of Sales')">
                                    <a href="#" :class="{ 'text-violet-600': selectedCategory === 'Point of Sales' }">
                                        Point of Sales
                                    </a>
                                </li>
                                <li class="mb-1 hover:bg-violet-100 hover:rounded-md py-1 px-3 cursor-pointer"
                                    @click.prevent="filterByCategory('Healthcare')">
                                    <a href="#" :class="{ 'text-violet-600': selectedCategory === 'Healthcare' }">
                                        Healthcare
                                    </a>
                                </li>
                                <li class="mb-1 hover:bg-violet-100 hover:rounded-md py-1 px-3 cursor-pointer"
                                    @click.prevent="filterByCategory('Education')">
                                    <a href="#" :class="{ 'text-violet-600': selectedCategory === 'Education' }">
                                        Education
                                    </a>
                                </li>
                                <li class="mb-1 hover:bg-violet-100 hover:rounded-md py-1 px-3 cursor-pointer"
                                    @click.prevent="filterByCategory('Technology')">
                                    <a href="#" :class="{ 'text-violet-600': selectedCategory === 'Technology' }">
                                        Technology
                                    </a>
                                </li>
                                <li class="mb-1 hover:bg-violet-100 hover:rounded-md py-1 px-3 cursor-pointer"
                                    @click.prevent="filterByCategory('Logistics')">
                                    <a href="#" :class="{ 'text-violet-600': selectedCategory === 'Logistics' }">
                                        Logistics
                                    </a>
                                </li>
                                <li class="mb-1 hover:bg-violet-100 hover:rounded-md py-1 px-3 cursor-pointer"
                                    @click.prevent="filterByCategory('Customer Support')">
                                    <a href="#" :class="{ 'text-violet-600': selectedCategory === 'Customer Support' }">
                                        Customer Support
                                    </a>
                                </li>
                            </ul>

                        </div>
                        <div class="w-full px-4 sm:px-6 py-4 md:py-6 mt-2 mx-auto">
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
                                <Card v-for="(card, index) in paginatedFilteredCards" :key="index" :title="card.title"
                                    :description="card.description" :status="card.status" :buttonText="card.buttonText"
                                    :iconClass="card.iconClass" @click="handleCardClick(card.title)">
                                </Card>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <RouterView v-slot="{ Component }">
                <Transition name="fade" mode="out-in">
                    <Component :is="Component" />
                </Transition>
            </RouterView>
        </div>
    </div>
</template>