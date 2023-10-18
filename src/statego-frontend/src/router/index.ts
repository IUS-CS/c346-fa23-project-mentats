import { createRouter, createWebHistory } from 'vue-router';
import LandingView from '../views/LandingView.vue';
import AboutView from '../views/AboutView.vue';
import SessionView from '../views/SessionView.vue';
import LogSessionView from '../views/LogSessionView.vue';
import SignupView from '../views/SignupView.vue';
import LogInViewVue from '@/views/LogInView.vue';

const router = createRouter({
   history: createWebHistory(import.meta.env.BASE_URL),
   routes: [
      //route to the homepage
      {
         path: '/',
         name: 'home',
         component: LandingView,
      },
      //route to the about page
      {
         path: '/about',
         name: 'About',
         component: AboutView,
      },
      //route to Session Log page
      {
         path: '/logsession',
         name: 'LogSession',
         component: LogSessionView,
      },
      //route to Login page
      {
         path: '/login',
         name: 'LogIn',
         component: LogInViewVue,
      },
      {
         path: '/signup',
         name: 'Signup',
         component: SignupView,
      },
   ],
});

export default router;
