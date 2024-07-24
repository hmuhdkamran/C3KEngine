<script setup lang="ts">
</script>

<template>
    <div>
        <section class="header relative pt-16 items-center flex h-screen" style="max-height:860px">
            <div class="container mx-auto items-center flex flex-wrap">
                <div class="w-full md:w-8/12 lg:w-6/12 xl:w-6/12 px-4">
                    <div class="pt-32 sm:pt-0">
                        <h2 class="font-semibold text-4xl text-gray-600">A progressive framework.</h2>
                        <p class="mt-4 text-lg leading-relaxed text-gray-500">C3K Engine is a monolithical rust and vue 3,
                            it is using
                            <a href="https://tailwindcss.com/?ref=creativetim" class="text-gray-600"
                                target="_blank">TailwindCSS</a>.</p>
                    </div>
                </div>
            </div><img class="absolute top-0 b-auto right-0 pt-16 sm:w-6/12 -mt-48 sm:mt-0 w-10/12"
                src="https://www.creative-tim.com/learning-lab/tailwind-starter-kit/img/ill_header_3.png" alt="..."
                style="max-height:860px">
        </section>

        <section class="pb-16 bg-gray-200 relative pt-32">
            <div class="-mt-20 top-0 bottom-auto left-0 right-0 w-full absolute" style="height:80px"><svg
                    class="absolute bottom-0 overflow-hidden" xmlns="http://www.w3.org/2000/svg"
                    preserveAspectRatio="none" version="1.1" viewBox="0 0 2560 100" x="0" y="0">
                    <polygon class="text-gray-200 fill-current" points="2560 0 2560 100 0 100"></polygon>
                </svg></div>
            <div class="container mx-auto">
                <div
                    class="flex flex-wrap justify-center bg-white shadow-xl rounded-lg -mt-64 py-16 px-12 relative z-10">
                    <div class="w-full text-center lg:w-8/12">
                        <p class="text-4xl text-center"><span role="img" aria-label="love">😍</span></p>
                        <h3 class="font-semibold text-3xl">Do you love this Starter Kit?</h3>
                        <p class="text-gray-500 text-lg leading-relaxed mt-4 mb-4">Cause if you do, it can be yours
                            now. Hit the buttons below to navigate to get the Free version for your next project. Build
                            a new web app or give an old project a new look!</p>
                        <div class="sm:block flex flex-col mt-10"><a
                                class="get-started text-white font-bold px-6 py-4 rounded outline-none focus:outline-none mr-1 mb-2 bg-pink-500 active:bg-pink-600 uppercase text-sm shadow hover:shadow-lg ease-linear transition-all duration-150"
                                href="/learning-lab/tailwind-starter-kit/documentation/quick-start">Get started</a><a
                                href="https://github.com/creativetimofficial/tailwind-starter-kit" target="_blank"
                                class="github-star sm:ml-1 text-white font-bold px-6 py-4 rounded outline-none focus:outline-none mr-1 mb-1 bg-gray-700 active:bg-gray-600 uppercase text-sm shadow hover:shadow-lg"><i
                                    class="fab fa-github text-lg mr-1"></i><span>Help With a Star</span></a></div>
                        <div class="text-center mt-16"></div>
                    </div>
                </div>
            </div>
        </section>
    </div>
</template>