<script setup lang="ts">
    import { RouterLink } from "vue-router";
    
</script>

<template>

    <nav class="flex w-full h-full items-center justify-center">

        <RouterLink class="p-5 hover:bg-slate-900 text-white" to="/">Home</RouterLink>
        <RouterLink class="p-5 hover:bg-slate-900 text-white" to="/about">About</RouterLink>
        
    </nav>


</template>