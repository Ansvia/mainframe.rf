<template>
  <div id="$param.service_name_camel_case$Detail">
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
  name: "$param.service_name_camel_case$Detail",
  components: {
    AnsTab
  },
  props: {
    $param.service_name_snake_case$Id: String
  },
  data() {
    return {
      d: {}
    };
  },
  created() {
    if (!this.$param.service_name_snake_case$Id) return;
    this.$$name_snake_case$
      .api()
      .privateApi.get(`/$param.service_name_snake_case$/v1/$param.service_name_snake_case$/info?id=${this.$param.service_name_snake_case$Id}`)
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
