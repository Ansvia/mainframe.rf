<template>
  <div class="home">
    <div style="float: left;">
      <sidebar-menu
        :menu="menu"
        @collapse="onCollapse"
        @itemClick="onItemClick"
        :collapsed="true"
        style="z-index: 1000;"
      />
    </div>

    <div class="dashboard-inner" v-bind:style="customMargin">
      <h1>{{ pageTitle }}</h1>

      <AnsTable
        v-if="currentPage['/dashboard']"
        data-source-url="/$param.service_name_snake_case$/v1/$param.service_name_snake_case$s"
        :columns="['ID', 'Name', 'Email']"
        :searchable="true"
        :withActionButton="true"
        :mapItemFunc="userListAllMapper2"
        :showDetailFunc="showDetail"
      ></AnsTable>

      <AnsTable
        v-if="currentPage['/dashboard/$param.service_name_snake_case$s']"
        data-source-url="/$param.service_name_snake_case$/v1/$param.service_name_snake_case$s"
        :columns="['ID', 'Name', 'Email', 'Phone', 'Active', 'Register']"
        :searchable="true"
        :withActionButton="true"
        :mapItemFunc="userListAllMapper"
        :showDetailFunc="showDetail"
      />

      <$param.service_name_pascal_case$Detail v-if="$route.path.startsWith('/dashboard/$param.service_name_snake_case$s/')" :$param.service_name_camel_case$Id="$route.params.id"/>
    </div>

    <notifications group="default" position="top center" classes="vue-notification" />
  </div>
</template>

<script>
// @ is an alias to /src
import AnsTable from "@/components/AnsTable.vue";
import $param.service_name_pascal_case$Detail from "@/components/$param.service_name_pascal_case$Detail.vue";

export default {
  name: "Dashboard",
  components: {
    AnsTable,
    $param.service_name_pascal_case$Detail
  },
  data() {
    return {
      collapsed: true,
      customMargin: {},
      currentPage: {},
      pageTitle: this.pageTitle,
      menu: [
        {
          header: true,
          title: "Main Navigation"
        },
        {
          href: "/dashboard",
          title: "Dashboard",
          icon: "fa fa-user"
        },
        {
          title: "$param.service_name_pascal_case$s",
          icon: "fa fa-users",
          href: "/dashboard/$param.service_name_snake_case$s"
        },
        {
          title: "Logout",
          icon: "fa fa-sign-out-alt"
        }
      ]
    };
  },
  created() {
    this.customMargin = {
      left: "70px",
      position: "absolute"
    };

    this.currentPage = {};
    this.$set(this.currentPage, this.$route.path, true);
    this.pageTitle = this.$router.history.current.name;

    this.startLoginChecker();
  },
  destroyed() {
    clearInterval(this.loginCheckerIval);
  },
  methods: {
    publicApiScope(){
      return this.$$name_snake_case$
        .api()
        .publicApi;
    },
    showDetail(item){
      this.$router.push("/dashboard/users/" + item.id);
    },
    txItemMap(item) {
      return item;
    },
    userListAllMapper(item) {
      return item;
    },
    userListAllMapper2(item) {
      return {
        id: item["id"],
        name: item["full_name"],
        email: item["email"]
      };
    },
    isCurrentPage(title) {
      return this.currentPage == title;
    },
    startLoginChecker() {
      var self = this;
      this.loginCheckerIval = setInterval(() => {
        this.$$name_snake_case$.isLoggedIn(loggedIn => {
          if (!loggedIn) {
            self.$router.replace("/");
          }
        });
      }, 3000);
    },
    onCollapse(state) {
      this.collapsed = state;
      this.customMargin = {
        left: this.collapsed ? "70px" : "370px",
        position: "absolute"
      };
    },
    onItemClick(_event, item) {
      if (item.title == 'Logout'){
        this.$dialog.confirm("Are you sure to logout?")
          .then((_dialog) => {
            this.$$name_snake_case$.unauthorize();
          })
          .catch(()=>{});
      }
    }
  }
};
</script>


<style lang="less" scoped>
.dashboard-inner {
  width: 100%;
  transition: all 0.1s ease-in-out;
  -webkit-transition: all 0.1s ease-in-out; /** Chrome & Safari **/
  -moz-transition: all 0.1s ease-in-out; /** Firefox **/
  -o-transition: all 0.1s ease-in-out; /** Opera **/
}
</style>
