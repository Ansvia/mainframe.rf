<template>
  <div id="AccountDetail">
    <AnsTab>
      <div class="ui grid">
        <div class="six wide column">
          <table class="ui celled table">
            <tbody>
              <tr>
                <td data-label="ID">ID:</td>
                <td class="value">{{d.id}}</td>
              </tr>
              <tr>
                <td data-label="Name">Full name:</td>
                <td class="value">{{d.full_name}}</td>
              </tr>
              <tr>
                <td data-label="Email">Email:</td>
                <td class="value">{{d.email}}</td>
              </tr>
              <tr>
                <td data-label="Phone">Phone:</td>
                <td class="value">{{d.phone_num}}</td>
              </tr>
              <tr>
                <td data-label="Active">Active:</td>
                <td class="value">{{d.active ? "YES" : "NO"}}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </AnsTab>
  </div>
</template>

<script>
import AnsTab from "@/components/AnsTab";

export default {
  name: "AccountDetail",
  components: {
    AnsTab
  },
  props: {
    accountId: String
  },
  data() {
    return {
      d: {}
    };
  },
  created() {
    if (!this.accountId) return;
    this.$$name_snake_case$
      .api()
      .privateApi.get(`/$param.service_name_snake_case$/v1/account/info?id=${this.accountId}`)
      .then(resp => {
        this.d = resp.data.result;
      });
  },
  methods: {
  }
};
</script>

<style lang="less" scoped>
.value {
  font-weight: bold;
}
tr td:first-child {
  text-align: right !important;
}
</style>
