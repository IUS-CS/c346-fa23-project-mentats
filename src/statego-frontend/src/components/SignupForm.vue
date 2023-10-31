<script setup lang="ts">
import { ref, type Ref } from 'vue';

const username: Ref<string> = ref('');
const email: Ref<string> = ref('');
const password: Ref<string> = ref('');
const firstname: Ref<string> = ref('');
const lastname: Ref<string> = ref('');
const checkpassword: Ref<string> = ref('');

function submit() {
   // make post request to backend
   // if success redirect to dashboard
}
</script>

<template>
   <div class="flex w-5/6 flex-col items-center rounded-md border-t-8 border-violet-500 bg-gray-100 shadow-lg md:w-1/4">
      <h1 class="p-3 font-Nunito-Sans text-3xl font-bold">Sign Up</h1>
      <form class="flex w-full flex-col items-center justify-center gap-y-4">
         <!-- email addrees field -->
         <div class="flex w-4/5 flex-col">
            <p class="font-Nunito-Sans text-sm font-light">Email</p>
            <input class="h-9 w-full rounded-md border border-gray-300 bg-gray-50 font-Nunito-Sans" type="text" name="Email" v-model="email" />
         </div>
         <!-- username field -->
         <div class="flex w-4/5 flex-col">
            <p class="font-Nunito-Sans text-sm font-light">Username</p>
            <input class="h-9 w-full rounded-md border border-gray-300 bg-gray-50" type="text" name="Username" v-model="username" />
         </div>
         <!-- name fields -->
         <div class="flex w-4/5 flex-col gap-5 md:flex-row">
            <!-- first name -->
            <div class="flex-initial flex-col md:w-1/2">
               <p class="font-Nunito-Sans text-sm font-light">First Name</p>
               <input class="h-9 w-full rounded-md border border-gray-300 bg-gray-50" type="text" name="Fist Name" v-model="firstname" />
            </div>
            <!-- last name -->
            <div class="flex-initial flex-col md:w-1/2">
               <p class="font-Nunito-Sans text-sm font-light">Last Name</p>
               <input class="h-9 w-full rounded-md border border-gray-300 bg-gray-50" type="text" name="Last Name" v-model="lastname" />
            </div>
         </div>
         <!-- password field -->
         <div class="flex w-4/5 flex-col">
            <p class="font-Nunito-Sans text-sm font-light">Password</p>
            <input class="h-9 w-full rounded-md border border-gray-300 bg-gray-50" type="password" name="Password" v-model="password" />
         </div>
         <!-- confirm password field -->
         <div class="flex w-4/5 flex-col">
            <p class="font-Nunito-Sans text-sm font-light">Confirm Password</p>
            <input class="h-9 w-full rounded-md border border-gray-300 bg-gray-50" type="password" name="Confirm Password" v-model="checkpassword" />
         </div>
         <div class="flex w-4/5 flex-col pb-6">
            <button class="h-9 w-28 rounded-md bg-violet-500 px-6 font-Nunito-Sans font-bold shadow-md" type="submit" @click.prevent="submit">
               Submit
            </button>
            <p class="pt-2 font-Nunito-Sans text-sm">
               Already have an account?
               <RouterLink class="text-violet-400 underline" to="/login"> Login! </RouterLink>
            </p>
         </div>
      </form>
   </div>
</template>
