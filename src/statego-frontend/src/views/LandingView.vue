<script setup lang="ts">
import { useMouseInElement } from '@vueuse/core';
import { computed, ref } from 'vue';
import type { RouterLink } from 'vue-router';
// @ts-ignore
import { useElementTransform } from '/node_modules/@vueuse/motion/dist';

const target = ref();
const logo1 = ref<HTMLElement>();
const logo2 = ref<HTMLElement>();
const logo3 = ref<HTMLElement>();
const logo4 = ref<HTMLElement>();

const mouseRel = useMouseInElement(target);
const transform2 = useElementTransform(logo2);
const transform3 = useElementTransform(logo3);
const transform4 = useElementTransform(logo4);

const scale = {
   x: computed(() => (mouseRel.x.value - mouseRel.elementWidth.value / 2) / 500),
   y: computed(() => (mouseRel.y.value - mouseRel.elementHeight.value / 2) / 500),
};

transform2.transform.translateX = computed(() => 20 * scale.x.value);
transform3.transform.translateX = computed(() => 40 * scale.x.value);
transform4.transform.translateX = computed(() => 60 * scale.x.value);

transform2.transform.translateY = computed(() => 20 * scale.y.value);
transform3.transform.translateY = computed(() => 30 * scale.y.value);
transform4.transform.translateY = computed(() => 40 * scale.y.value);
</script>

<template>
   <div class="flex h-full w-full flex-col items-center justify-center lg:flex-row">
      <div ref="target" class="flex h-full w-4/5 items-center justify-center lg:w-1/2 lg:p-10">
         <img ref="logo4" src="/statego-logo-v2-med.svg" alt="Statego Logo" class="fixed" />
         <img ref="logo3" src="/statego-logo-v2-med-dark.svg" alt="Statego Logo" class="fixed" />
         <img ref="logo2" src="/statego-logo-v2-dark.svg" alt="Statego Logo" class="fixed" />
         <img ref="logo1" src="/statego-logo-v2.svg" alt="Statego Logo" class="fixed" />
      </div>
      <div class="z-10 flex w-4/5 flex-col pt-10 lg:w-1/2 lg:pt-0">
         <h1 class="font-Nunito-Sans text-5xl md:text-9xl">STATEGO</h1>
         <p class="font-Nunito-Sans text-2xl">A board game statistics tracker and anaysis tool</p>
         <div class="flex w-full flex-row gap-x-6">
            <RouterLink to="/signup" class="flex h-10 w-24 items-center justify-center rounded-md bg-violet-500 font-Nunito-Sans font-bold">
               Signup
            </RouterLink>
            <RouterLink to="/about" class="flex h-10 w-24 items-center justify-center rounded-md border border-violet-500 font-Nunito-Sans font-bold">
               About
            </RouterLink>
         </div>
      </div>
   </div>
</template>
