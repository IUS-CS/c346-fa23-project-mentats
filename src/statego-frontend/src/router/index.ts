import { createRouter, createWebHistory } from 'vue-router'
import LandingView from '../views/LandingView.vue'
import AboutView from '../views/AboutView.vue'
import SessionView from '../views/SessionView.vue'
import LogSessionView from '../views/LogSessionView.vue'
import SignupForm from '../components/SignupForm.vue'

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
    {
      path: '/session',
      name: 'Session',
      component: SessionView
    },
    {
      path: '/logsession',
      name: 'LogSession',
      component: LogSessionView
    },
    {
      path: '/signup',
      name: 'Signup',
      component: SignupForm
    }
  ]
})

export default router
