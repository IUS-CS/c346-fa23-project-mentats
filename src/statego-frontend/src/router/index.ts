import { createRouter, createWebHistory } from 'vue-router'
import LandingView from '../views/LandingView.vue'
import AboutView from '../views/AboutView.vue'
import LogSessionView from '../views/LogSessionView.vue'
import LogInViewVue from '@/views/LogInView.vue'



const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    //route to the homepage
    {
      path: '/',
      name: 'home',
      component: LandingView
    },
    //route to the about page
    {
      path: '/about',
      name: 'About',
      component: AboutView
    },
    //route to Session Log page
    {
      path: '/logsession',
      name: 'LogSession',
      component: LogSessionView
    },
    //route to Login page
    {
      path: '/login',
      name: 'LogIn',
      component: LogInViewVue
    }
  ]
})

export default router
