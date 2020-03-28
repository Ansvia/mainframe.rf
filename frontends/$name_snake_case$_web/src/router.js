import Vue from 'vue'
import Router from 'vue-router'
import Home from './views/Home.vue'
import Dashboard from './views/Dashboard.vue'
import NotFound from './views/NotFound.vue'
import Example from './views/Example.vue'

Vue.use(Router)

const defaultTitle = "$name$"
const titleDesc = ` | ${defaultTitle}`

let router = new Router({
  mode: 'history',
  base: process.env.BASE_URL,
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home,
      meta: {
        title: 'Home' + titleDesc,
      },
    },
    {
      path: '/dashboard',
      name: 'Dashboard',
      component: Dashboard,
      meta: {
        title: 'Dashboard' + titleDesc,
      },
    },
    {
      path: '/dashboard/$param.service_name_snake_case$s',
      name: '$param.service_name_pascal_case$',
      component: Dashboard,
      meta: {
        title: 'Dashboard' + titleDesc,
      },
    },
    {
      path: '/dashboard/$param.service_name_snake_case$s/:id',
      name: '$param.service_name_pascal_case$ detail',
      component: Dashboard,
      meta: {
        title: 'Dashboard' + titleDesc,
      },
    },
    {
      path: '/example',
      name: 'example',
      component: Example,
      meta: {
        title: 'Example' + titleDesc,
      }
    },
    {
      path: '*',
      name: '404',
      component: NotFound,
      meta: {
        title: 'Oops! Not Found' + titleDesc,
      }
    }
  ],
  scrollBehavior () {
    return { x: 0, y: 0 }
  }
})

router.beforeEach((to, _from, next) => {
  to.matched.forEach(record => {
    document.title = record.meta.title || defaultTitle
  });

  next()
})

export default router
