<template>
  <div id="app">
    <div id="nav">
      <!-- <router-link to="/">Home</router-link> |
      <router-link to="/about">About</router-link> -->
    </div>
    <router-view :key="$route.path"/>
    <notifications group="auth" position="top center" classes="vue-notification" />
  </div>
</template>

<style lang="less">
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  // text-align: center;
  color: #2c3e50;
}
#nav {
  padding: 30px;
  a {
    font-weight: bold;
    color: #2c3e50;
    &.router-link-exact-active {
      color: #42b983;
    }
  }
}

/* NOTIFIKASI */
.vue-notification {
  padding: 10px;
  margin: 10px 5px 5px 0px;
  font-size: 15px; 
  color: #ffffff;
  background: #44A4FC;
  text-align: center;
  border-left: 0;
  border-radius: 10px;
 
  &.warn {
    background: #ffb648;
    color: black;
  }
 
  &.error {
    background: #E54D42;
  }
 
  &.success {
    background: #68CD86;
  }
}
</style>
