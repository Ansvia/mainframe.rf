import Vue from 'vue'
import VueSession from 'vue-session';
import VueSidebarMenu from "vue-sidebar-menu";
import 'vue-sidebar-menu/dist/vue-sidebar-menu.css'
import Notifications from 'vue-notification'

import App from './App.vue'
import router from './router'
import store from './store'
import $name_camel_case$ from './plugins/$name_kebab_case$';

// ----- Vuejs Dialog Stuff -------------
import VuejsDialog from "vuejs-dialog"
// import VuejsDialogMixin from "vuejs-dialog/vuejs-dialog-mixin.min.js"
 
// include the default style
import 'vuejs-dialog/dist/vuejs-dialog.min.css'
 
// Tell Vue to install the plugin.
Vue.use(VuejsDialog)
// ------- end of Vuejs Dialog Stuff ---------

import './registerServiceWorker'


Vue.config.productionTip = false

// Configure your base api endpoint for production here:
Vue.config.prodApiEndpoint = "http://api.$name_kebab_case$.com";

// Run mode ini menerima nilai:
// * `prod` - Apabila ingin menggunakan API dari server production.
// * `dev` - Apabila ingin menggunakan API dari server local atau docker (untuk development).
// * `mock` - Apabila ingin menggunakan API dari server mocking Apiary (untuk development).
Vue.config.runMode = "dev";

Vue.use(VueSession)
Vue.use(Notifications)
Vue.use($name_camel_case$)
Vue.use(VueSidebarMenu)

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')
