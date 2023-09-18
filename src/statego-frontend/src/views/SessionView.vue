<script setup lang="ts">
    let imgAlt: String = "Game Board"
</script>

<template>
    <div class="flex w-5/6 h-full bg-white">
        <!-- left col -->
        <div class="flex flex-col w-2/5 h-full items-center justify-center gap-16">
            <!-- images from session -->
            <div class="w-4/5 h-2/5">
                <img class="flex" :alt="imgAlt" src="https://placehold.co/600x400"/>
            </div>
            <!-- time and name -->
            <div class="w-4/5 h-2/5">
                <h1>Game Name</h1>
                <h2>Campeign</h2>
                <p>Date/Time</p>
                <p>Length</p>
            </div>
        </div>

        <!-- right col -->
        <div class="flex flex-col w-3/5 h-full items-center justify-center gap-16">
            <!-- players / winner -->
            <div class="w-4/5 h-2/5">

            </div>
            <!-- session notes -->
            <div class="w-4/5 h-2/5">

            </div>
        </div>
    </div>
</template>

<style scoped>
/* div{
    outline: 2px;
    outline-color: black;
    outline-style: solid;
} */
</style>