import { createRouter, createWebHistory } from 'vue-router';
import LandingView from '../views/LandingView.vue';
import AboutView from '../views/AboutView.vue';



const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: LandingView
    },
    {
      path: '/about',
      name: 'About',
      component: AboutView
    },
  ]
})

export default router
