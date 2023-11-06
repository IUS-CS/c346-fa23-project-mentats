<script setup lang="ts">
import { useMouseInElement } from '@vueuse/core';
import { computed, reactive, ref } from 'vue';
import type { RouterLink } from 'vue-router';
import { useElementTransform } from '/node_modules/@vueuse/motion/dist';

const target = ref();
const logo1 = ref<HTMLElement>();
const logo2 = ref<HTMLElement>();
const logo3 = ref<HTMLElement>();
const logo4 = ref<HTMLElement>();

const mouseRel = useMouseInElement(target);
const transform1 = useElementTransform(logo1);

const scale = {
   x: computed(() => (mouseRel.x.value - (mouseRel.elementWidth.value/2))/500),
   y: 1
}

transform1.transform.translateX = computed(() => 10 * scale.x.value)

</script>

<template>
   <div class="flex w-full h-full flex-col lg:flex-row justify-center items-center">
      <div ref="target" class="flex h-full w-4/5 lg:w-1/3 lg:p-10 items-center justify-center">
         <img ref="logo1" src="/statego-logo-v2.svg" alt="Statego Logo" class="fixed">
         <img ref="logo2" src="/statego-logo-v2.svg" alt="Statego Logo" class="fixed">
         <img ref="logo3" src="/statego-logo-v2.svg" alt="Statego Logo" class="fixed">
         <img ref="logo4" src="/statego-logo-v2.svg" alt="Statego Logo" class="fixed">
      </div>
      <div class="pt-10 lg:pt-0 flex flex-col justify-center items-center w-4/5 lg:w-2/3">
         <h1 class="font-Nunito-Sans text-5xl md:text-9xl">STATEGO</h1>
         <p class="font-Nunito-Sans text-2xl">A board game statistics tracker and anaysis tool{{ scale.x }}</p>
         <div>
            <RouterLink to="/signup">Signup</RouterLink>
            <RouterLink to="/about">About</RouterLink>
         </div>
      </div>
   </div>   
</template>