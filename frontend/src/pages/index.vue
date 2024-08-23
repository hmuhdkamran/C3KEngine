<script setup lang="ts">
import illustration from '@/assets/images/hero.png'
import image1 from '@/assets/images/image1.jpg'
import image2 from '@/assets/images/image2.jpg'
import image3 from '@/assets/images/image3.jpg'
import image4 from '@/assets/images/image4.jpg'
import avatar from '@/assets/images/avatar.jpg'

</script>

<template>
    <div>
        <section class="header relative items-center flex h-screen">
            <div class="container mx-auto items-center flex flex-wrap">
                <div class="absolute w-full lg:w-1/2 inset-y-0 lg:right-0 hidden lg:block">
                    <span class="absolute -left-12 top-24 w-40 h-40 rounded-full bg-gradient-to-r from-purple-500 
                    to-violet-500 blur-3xl opacity-50 transform rotate-45 lg:block hidden"></span>
                </div>
                <span class="w-3/12 lg:w-2/12 aspect-square bg-gradient-to-r from-violet-600 to-purple-400 
                absolute -top-5 lg:left-0 rounded-full blur-3xl opacity-50 transform rotate-45"></span>
                <div class="w-full md:w-8/12 lg:w-6/12 px-4 relative z-10">
                    <div class="pt-16 md:pt-24 lg:pt-32">
                        <h2 class="font-semibold text-3xl md:text-4xl lg:text-5xl xl:text-6xl text-gray-800">
                            Best <span class="bg-gradient-to-tr from-violet-800 to-violet-400 bg-clip-text text-transparent">
                                HR Software</span> to Manage Your Employees
                        </h2>
                        <p class="mt-4 md:mt-6 text-base md:text-lg lg:text-xl leading-relaxed text-gray-500 max-w-md">
                            A complete HR software & highly regarded On-Cloud & On-Premise HCMS that covers nearly the
                            entire scope of HR management while being strictly compliant with both local and global best
                            HR Practices.
                        </p>
                        <a class="mt-8 md:mt-12 group relative inline-flex items-center overflow-hidden rounded bg-gradient-to-r from-violet-600 to-indigo-500 px-6 md:px-8 py-2 md:py-3 text-white focus:outline-none focus:ring active:bg-violet-700"
                            href="#">
                            <span class="absolute -end-full transition-all group-hover:end-4">
                                <span class="icon-[heroicons--arrow-long-right] mt-2"
                                    style="width: 24px; height: 24px;"></span>
                            </span>
                            <span class="text-sm font-medium transition-all group-hover:me-4">View All Modules</span>
                        </a>
                    </div>
                </div>
                <div class="w-full lg:w-6/12 mt-10 lg:mt-0">
                    <div class="text-center lg:text-right lg:ml-auto">
                        <div class="relative inline-block z-10 pt-11 lg:pt-0">
                            <img :src="illustration" alt="hero" class="max-w-full lg:ml-auto" />
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <section class="py-2 bg-gray-200 relative sm:mt-36">
            <div class="-mt-20 top-0 bottom-auto left-0 right-0 w-full absolute" style="height:80px">
                <svg class="absolute bottom-0 overflow-hidden" xmlns="http://www.w3.org/2000/svg"
                    preserveAspectRatio="none" version="1.1" viewBox="0 0 2560 100" x="0" y="0">
                    <polygon class="text-gray-200 fill-current" points="2560 0 2560 100 0 100"></polygon>
                </svg>
            </div>
        </section>

        <section class="py-2 bg-gray-200">
            <div class="container mx-auto px-4">
                <div class="text-center py-4">
                    <p class="text-base font-semibold leading-6 text-violet-600 uppercase">Features</p>
                    <h2 class="text-2xl md:text-3xl font-extrabold mb-4 md:mb-6">Key Benefits of Our HRMS</h2>
                    <p class="text-gray-500">Simplify HR processes and boost productivity with our all-in-one HR
                        platform.</p>
                </div>
                <div class="grid gap-6 grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
                    <div class="bg-white p-6 rounded-lg shadow-lg text-center">
                        <span class="mb-4 inline-block p-3 rounded-lg bg-violet-100">
                            <span class="icon-[eos-icons--secure-data] w-10 h-10 text-violet-600"></span>
                        </span>
                        <h3 class="text-lg md:text-xl font-bold text-gray-800">Centralized HR Data</h3>
                        <p class="text-gray-500 mt-2">Keep all your employee records, payroll, and performance data in
                            one secure place.</p>
                    </div>
                    <div class="bg-white p-6 rounded-lg shadow-lg text-center">
                        <span class="mb-4 inline-block p-3 rounded-lg bg-violet-100">
                            <span class="icon-[mdi--recurring-payment] w-10 h-10 text-violet-600"></span>
                        </span>
                        <h3 class="text-lg md:text-xl font-bold text-gray-800">Automated Payroll</h3>
                        <p class="text-gray-500 mt-2">Ensure accuracy and timeliness in payroll processing with
                            automated calculations.</p>
                    </div>
                    <div class="bg-white p-6 rounded-lg shadow-lg text-center">
                        <span class="mb-4 inline-block p-3 rounded-lg bg-violet-100">
                            <span class="icon-[clarity--employee-group-solid] w-10 h-10 text-violet-600"></span>
                        </span>
                        <h3 class="text-lg md:text-xl font-bold text-gray-800">Employee Self-Service</h3>
                        <p class="text-gray-500 mt-2">Enable employees to manage personal data, requests, and time-off
                            seamlessly.</p>
                    </div>
                </div>
            </div>
        </section>

        <section>
            <div class="mt-16 bg-gray-50 radius-for-skewed">
                <div class="container mx-auto px-4">
                    <div class="flex flex-wrap items-center">
                        <div class="w-full lg:w-1/2 mb-12 lg:mb-0">
                            <div class="max-w-md lg:mx-auto">
                                <span class="text-violet-600 font-bold">HRMS Solutions</span>
                                <h2 class="my-2 text-4xl lg:text-5xl font-bold font-heading">Streamline Your HR
                                    Processes</h2>
                                <p class="mb-6 text-violet-500 leading-loose">Our HRMS software simplifies HR tasks,
                                    enhances efficiency, and provides comprehensive data security.</p>
                                <ul class="text-gray-500 font-bold">
                                    <li class="flex mb-4">
                                        <span class="icon-[ep--success-filled] mr-2 w-6 h-6 text-violet-600"></span>
                                        <span>Efficient Employee Management</span>
                                    </li>
                                    <li class="flex mb-4">
                                        <span class="icon-[ep--success-filled] mr-2 w-6 h-6 text-violet-600"></span>
                                        <span>Automated Payroll Processing</span>
                                    </li>
                                    <li class="flex mb-4">
                                        <span class="icon-[ep--success-filled] mr-2 w-6 h-6 text-violet-600"></span>
                                        <span>Comprehensive Reporting</span>
                                    </li>
                                </ul>
                            </div>
                        </div>
                        <div class="w-full lg:w-1/2 flex flex-wrap -mx-4">
                            <div class="mb-8 lg:mb-0 w-full md:w-1/2 px-4">
                                <div class="mb-8 py-6 pl-6 pr-4 shadow rounded bg-white">
                                    <span class="mb-4 inline-block p-3 rounded-lg bg-violet-100">
                                        <span class="icon-[tabler--photo] w-10 h-10 text-violet-600"></span>
                                    </span>
                                    <h4 class="mb-2 text-2xl font-bold font-heading">Employee Self-Service</h4>
                                    <p class="text-gray-500 leading-loose">Empower employees with self-service features
                                        for personal data management.</p>
                                </div>
                                <div class="py-6 pl-6 pr-4 shadow rounded bg-white">
                                    <span class="mb-4 inline-block p-3 rounded-lg bg-violet-100">
                                        <span class="icon-[fluent--toolbox-32-filled] w-10 h-10 text-violet-600"></span>
                                    </span>
                                    <h4 class="mb-2 text-2xl font-bold font-heading">Advanced Recruitment Tools</h4>
                                    <p class="text-gray-500 leading-loose">Streamline hiring with powerful recruitment
                                        and onboarding features.</p>
                                </div>
                            </div>
                            <div class="w-full md:w-1/2 lg:mt-20 px-4">
                                <div class="mb-8 py-6 pl-6 pr-4 shadow rounded-lg bg-white">
                                    <span class="mb-4 inline-block p-3 rounded bg-violet-100">
                                        <span
                                            class="icon-[streamline--star-badge-solid] w-10 h-10 text-violet-600"></span>
                                    </span>
                                    <h4 class="mb-2 text-2xl font-bold font-heading">Training and Development</h4>
                                    <p class="text-gray-500 leading-loose">Enhance employee skills with integrated
                                        training and development modules.</p>
                                </div>
                                <div class="py-6 pl-6 pr-4 shadow rounded-lg bg-white">
                                    <span class="mb-4 inline-block p-3 rounded bg-violet-100">
                                        <span class="icon-[mdi--tick-decagram] w-10 h-10 text-violet-600"></span>
                                    </span>
                                    <h4 class="mb-2 text-2xl font-bold font-heading">Compliance Management</h4>
                                    <p class="text-gray-500 leading-loose">Stay compliant with automated policy updates
                                        and legal requirements.</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <section>
            <div class="relative p-4 px-6 mx-auto bg-white dark:bg-gray-800 sm:px-6 md:px-8 lg:px-8 py-26 lg:mt-20">
                <div class="container mx-auto px-4">
                    <div class="lg:grid lg:grid-flow-row-dense lg:grid-cols-1 lg:gap-8 lg:items-center">
                        <div class="ml-16 lg:col-start-2 lg:max-w-2xl lg:ml-36">
                            <p class="text-base font-semibold leading-6 text-violet-600 uppercase">
                                Collaborative HRMS
                            </p>
                            <h4
                                class="mt-2 text-2xl font-extrabold leading-8 text-gray-900 dark:text-white sm:text-3xl sm:leading-9">
                                Interactivity and Efficiency for Your HR Team
                            </h4>
                            <p class="mt-4 text-lg leading-6 text-gray-500 dark:text-gray-300">
                                Create a seamless and powerful collaborative space for your team. Track, share, and
                                measure
                                efficiently with full control. It's never been simpler and more effective.
                            </p>
                            <ul class="gap-6 mt-8 md:grid md:grid-cols-2">
                                <li class="mt-6 md:mt-0">
                                    <div class="flex">
                                        <span
                                            class="flex items-center justify-center flex-shrink-0 w-6 h-6 text-violet-800 bg-violet-100 rounded-full dark:text-violet-500 drark:bg-transparent">
                                            <span class="icon-[charm--tick] w-4 h-4"></span>
                                        </span>
                                        <span
                                            class="ml-4 text-base font-medium leading-6 text-gray-500 dark:text-gray-200">
                                            Real-time Updates
                                        </span>
                                    </div>
                                </li>
                                <li class="mt-6 md:mt-0">
                                    <div class="flex">
                                        <span
                                            class="flex items-center justify-center flex-shrink-0 w-6 h-6 text-violet-800 bg-violet-100 rounded-full dark:text-violet-500 drark:bg-transparent">
                                            <span class="icon-[charm--tick] w-4 h-4"></span>
                                        </span>
                                        <span
                                            class="ml-4 text-base font-medium leading-6 text-gray-500 dark:text-gray-200">
                                            Comprehensive Data Tracking
                                        </span>
                                    </div>
                                </li>
                                <li class="mt-6 md:mt-0">
                                    <div class="flex">
                                        <span
                                            class="flex items-center justify-center flex-shrink-0 w-6 h-6 text-violet-800 bg-violet-100 rounded-full dark:text-violet-500 drark:bg-transparent">
                                            <span class="icon-[charm--tick] w-4 h-4"></span>
                                        </span>
                                        <span
                                            class="ml-4 text-base font-medium leading-6 text-gray-500 dark:text-gray-200">
                                            24/7 Support
                                        </span>
                                    </div>
                                </li>
                                <li class="mt-6 md:mt-0">
                                    <div class="flex">
                                        <span
                                            class="flex items-center justify-center flex-shrink-0 w-6 h-6 text-violet-800 bg-violet-100 rounded-full dark:text-violet-500 drark:bg-transparent">
                                            <span class="icon-[charm--tick] w-4 h-4"></span>
                                        </span>
                                        <span
                                            class="ml-4 text-base font-medium leading-6 text-gray-500 dark:text-gray-200">
                                            Tips to Optimize Work Time
                                        </span>
                                    </div>
                                </li>
                            </ul>
                        </div>
                        <div class="relative mt-12 lg:mt-0 space-y-4">
                            <div class="flex items-end justify-center space-x-4 sm:justify-start">
                                <img class="w-28 sm:w-32 md:w-56 lg:w-72 rounded-lg shadow-lg" :src="image3" alt="1" />
                                <img class="w-36 sm:w-40 md:w-64 lg:w-80 rounded-lg shadow-lg" :src="image2" alt="2" />
                            </div>
                            <div class="flex items-start justify-center space-x-4 sm:justify-start">
                                <img class="w-36 sm:w-40 md:w-64 lg:w-80 rounded-lg shadow-lg" :src="image4" alt="4" />
                                <img class="w-28 sm:w-32 md:w-56 lg:w-72 rounded-lg shadow-lg" :src="image1" alt="3" />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <div class="container mx-auto my-20 px-6">
            <section class="text-gray-800">
                <div class="text-center mb-16">
                    <p class="text-violet-500 uppercase font-bold text-sm mb-4">Features</p>
                    <h2 class="text-4xl font-extrabold mb-6">Why is it so great?</h2>
                    <p class="text-gray-600 text-lg mb-12">
                        Explore the exceptional features that make our HRMS software a must-have for your business. From
                        seamless integration to real-time analytics, our solution has it all.
                    </p>
                </div>
                <div class="grid md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-8">
                    <div class="flex items-start space-x-4 mb-12">
                        <div class="flex-shrink-0 bg-violet-100 p-3 rounded-full">
                            <span class="icon-[mdi--tick-circle] w-7 h-7 text-violet-600"></span>
                        </div>
                        <div>
                            <p class="text-lg font-semibold mb-1">24/7 Support</p>
                            <p class="text-gray-500">Our support team is always available to help with any queries or
                                issues you might have.</p>
                        </div>
                    </div>
                    <div class="flex items-start space-x-4 mb-12">
                        <div class="flex-shrink-0 bg-violet-100 p-3 rounded-full">
                            <span class="icon-[mdi--tick-circle] w-7 h-7 text-violet-600"></span>
                        </div>
                        <div>
                            <p class="text-lg font-semibold mb-1">Employee Tracking</p>
                            <p class="text-gray-500">Keep track of employee performance and progress effortlessly.</p>
                        </div>
                    </div>
                    <div class="flex items-start space-x-4 mb-12">
                        <div class="flex-shrink-0 bg-violet-100 p-3 rounded-full">
                            <span class="icon-[mdi--tick-circle] w-7 h-7 text-violet-600"></span>
                        </div>
                        <div>
                            <p class="text-lg font-semibold mb-1">Comprehensive Reporting</p>
                            <p class="text-gray-500">Generate detailed reports to gain deeper insights into your
                                workforce.</p>
                        </div>
                    </div>
                    <div class="flex items-start space-x-4 mb-12">
                        <div class="flex-shrink-0 bg-violet-100 p-3 rounded-full">
                            <span class="icon-[mdi--tick-circle] w-7 h-7 text-violet-600"></span>
                        </div>
                        <div>
                            <p class="text-lg font-semibold mb-1">Advanced Analytics</p>
                            <p class="text-gray-500">Leverage data-driven insights to make well-informed HR decisions.
                            </p>
                        </div>
                    </div>
                    <div class="flex items-start space-x-4 mb-12">
                        <div class="flex-shrink-0 bg-violet-100 p-3 rounded-full">
                            <span class="icon-[mdi--tick-circle] w-7 h-7 text-violet-600"></span>
                        </div>
                        <div>
                            <p class="text-lg font-semibold mb-1">Community Support</p>
                            <p class="text-gray-500">Join a vast community of users and get help when needed.</p>
                        </div>
                    </div>
                    <div class="flex items-start space-x-4 mb-12">
                        <div class="flex-shrink-0 bg-violet-100 p-3 rounded-full">
                            <span class="icon-[mdi--tick-circle] w-7 h-7 text-violet-600"></span>
                        </div>
                        <div>
                            <p class="text-lg font-semibold mb-1">Seamless Integration</p>
                            <p class="text-gray-500">Easily integrate with existing systems and tools.</p>
                        </div>
                    </div>
                    <div class="flex items-start space-x-4 mb-12">
                        <div class="flex-shrink-0 bg-violet-100 p-3 rounded-full">
                            <span class="icon-[mdi--tick-circle] w-7 h-7 text-violet-600"></span>
                        </div>
                        <div>
                            <p class="text-lg font-semibold mb-1">Customizable Dashboards</p>
                            <p class="text-gray-500">Tailor dashboards to suit your specific needs and preferences.</p>
                        </div>
                    </div>
                    <div class="flex items-start space-x-4 mb-12">
                        <div class="flex-shrink-0 bg-violet-100 p-3 rounded-full">
                            <span class="icon-[mdi--tick-circle] w-7 h-7 text-violet-600"></span>
                        </div>
                        <div>
                            <p class="text-lg font-semibold mb-1">Intuitive User Interface</p>
                            <p class="text-gray-500">Enjoy a user-friendly interface designed for ease of use and
                                efficiency.</p>
                        </div>
                    </div>
                </div>
            </section>
        </div>

        <div class="bg-white dark:bg-gray-800 py-16 lg:py-20">
            <div class="container mx-auto px-4 sm:px-6 lg:px-8">
                <h2 class="pb-4 text-3xl md:text-5xl font-bold text-gray-900 dark:text-white text-center">Get Started
                    with Our HRMS Today</h2>
                <p class="mx-auto max-w-3xl pb-8 text-gray-700 dark:text-gray-300 text-lg md:text-xl text-center">
                    Experience a seamless transition to our HRMS software. With intuitive features and user-friendly
                    design, our platform simplifies HR management, making it easier for your team to stay organized and
                    efficient.
                </p>
                <div class="lg:flex items-start justify-center lg:space-x-12">
                    <div class="lg:max-w-xl space-y-4 text-lg md:text-xl leading-7">
                        <div class="flex items-center space-x-3 text-gray-700 dark:text-gray-300 font-semibold">
                            <div class="shrink-0 text-violet-500">
                                <span class="icon-[mdi--tick-decagram]" style="width: 36px; height: 36px;"></span>
                            </div>
                            <div>Free to get started with comprehensive features</div>
                        </div>
                        <div class="flex items-center space-x-3 text-gray-700 dark:text-gray-300 font-semibold">
                            <div class="shrink-0 text-violet-500">
                                <span class="icon-[mdi--tick-decagram]" style="width: 36px; height: 36px;"></span>
                            </div>
                            <div>Easy migration checklist</div>
                        </div>
                        <div class="flex items-center space-x-3 text-gray-700 dark:text-gray-300 font-semibold">
                            <div class="shrink-0 text-violet-500">
                                <span class="icon-[mdi--tick-decagram]" style="width: 36px; height: 36px;"></span>
                            </div>
                            <div>Personalized support from our team</div>
                        </div>
                    </div>

                    <div
                        class="lg:border-l border-dashed border-gray-400 lg:px-12 w-full lg:max-w-lg text-center sm:text-left">
                        <div class="py-6 text-xl lg:text-2xl leading-7 lg:leading-8">
                            <img :src="avatar" width="150" height="150"
                                class="block w-32 h-32 rounded-full mx-auto sm:mx-0" alt="Customer Testimonial"
                                loading="lazy">
                            <p class="flex-grow py-6 text-gray-600 dark:text-gray-300">
                                “Adopting this HRMS was straightforward and effective. Its intuitive design and powerful
                                features have revolutionized our HR management.”
                            </p>
                            <div class="text-xl text-gray-700 dark:text-gray-100 font-medium">Omar Ahmed</div>
                            <div class="text-lg text-gray-500 dark:text-gray-400">HR Manager at Sefam</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>