import Vue from 'vue'
import Router from 'vue-router'
import Home from './views/Home.vue'
import Dashboard from './views/Dashboard.vue'

Vue.use(Router)

export default new Router({
  mode: 'history',
  base: process.env.BASE_URL,
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home
    },
    {
      path: '/dashboard',
      name: 'Dashboard',
      component: Dashboard
    },
    {
      path: '/dashboard/$param.service_name_snake_case$s',
      name: '$param.service_name$',
      component: Dashboard
    },
    {
      path: '/dashboard/$param.service_name_snake_case$s/:id',
      name: '$param.service_name$',
      component: Dashboard
    }
  ]
})
