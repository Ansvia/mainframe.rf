import { ApiClient, Crypto } from "../../../../libs/$name_kebab_case$-client-js";

// <% if param.with_protobuf %>
import * as protos from '../proto/stubs';
// <% endif %>

export default class $name_camel_case$ {
  static install(Vue) {

    var api;

    const env = process.env

    if (Vue.config.runMode == "prod") {
      api = new ApiClient(env.VUE_APP_PROD_PUBLIC_URL,
        env.VUE_APP_PROD_PRIVATE_URL);
    } else if (Vue.config.runMode == "dev") {
      api = new ApiClient(env.VUE_APP_DEV_PUBLIC_URL,
        env.VUE_APP_DEV_PRIVATE_URL);
    } else if (Vue.config.runMode == "mock") {
      api = new ApiClient(env.VUE_APP_MOCK_URL,
        env.VUE_APP_MOCK_URL);
    } else {
      throw "Unknown mode: " + Vue.config.runMode
    }

    updateSession();

    // var $session = Vue.prototype.$session;
    function session() {
      return Vue.prototype.$session;
    }

    function updateSession() {
      var token = session().get("token");
      api.publicApi.defaults.headers["X-Access-Token"] = token;
      api.privateApi.defaults.headers["X-Access-Token"] = token;
    }

    Vue.prototype.$$name_snake_case$ = {
      crypto() {
        return Crypto;
      },
      login(email, phone, password) {
        // <% if param.password_crypt_algo == "sha256" %>
        var passhash = Crypto.getPasshash(password);
        console.log("passhash: " + passhash);
        var emailOrPhone = email ? email : phone;
        var data = {
          "email": emailOrPhone,
          "phone": phone,
          "passhash": passhash
        };
        // <% endif %>

        // <% if param.password_crypt_algo == "bcrypt" %>
        var emailOrPhone = email ? email : phone;
        var data = {
          "email": emailOrPhone,
          "phone": phone,
          "password": password
        }
        // <% endif %>

        return api.publicApi.post("/auth/v1/authorize", data)
          .then((resp) => {
            if (resp.data.code == 0) {
              session().set("token", resp.data.result.token);
              updateSession(resp.data.result.token);
              this.load$param.service_name_camel_case$Key();
            }
            return resp;
          });
      },
      unauthorize() {
        console.log("unauthorize");
        session().remove("token");
        updateSession();
        return api.publicApi.post("/api/auth/v1/unauthorize", {});
      },
      isLoggedIn(cb) {
        this.getMeInfo().then((resp) => {
          if (resp.status != 200 || (resp.data.status == "error" && resp.data.code != 0)) {
            cb(false)
          } else {
            cb(true)
          }
        }).catch((_e) => cb(false))
      },
      getMeInfo() {
        return api.publicApi.get("/$param.service_name_snake_case$/v1/me/info");
      },

      // Fetch current $param.service_name_snake_case$ key-pair.
      load$param.service_name_camel_case$Key() {
        return api.publicApi.get("/auth/v1/get_key")
          .then((resp) => {
            console.log("$param.service_name_snake_case$ key loaded.");
            session().set("pk", resp.data.result.pub_key);
            session().set("sk", resp.data.result.secret_key);
          }).catch(_e => {
            // this.$notify({
            //   group: "default",
            //   type: "error",
            //   title: "Error",
            //   text: "Cannot load keys"
            // });
          });
      },

      // Mendapatkan current user key pair dari local session.
      getKeys() {
        var pk = session().get("pk");
        var sk = session().get("sk");
        return {
          pubKey: Buffer.from(pk, 'hex'),
          secretKey: Buffer.from(sk, 'hex'),
        }
      },
      // credit$param.service_name_camel_case$Balance($param.service_name_snake_case$Id, amount) {
      //   var credit = new protos.$param.service_name_snake_case$.Credit({
      //     $param.service_name_snake_case$: $param.service_name_snake_case$Id,
      //     amount: parseFloat(amount),
      //     timestamp: this.now(),
      //     seed: this.generateSeed()
      //   });

      //   var buffer = protos.$param.service_name_snake_case$.Credit.encode(credit).finish();
      //   let keys = this.getKeys();
      //   var signature = Crypto.sign(buffer, keys.pubKey, keys.secretKey);

      //   var data = {
      //     body: protos.$param.service_name_snake_case$.Credit.toObject(credit),
      //     signature: signature
      //   };
      //   return api.privateApi.post("/$param.service_name_snake_case$/v1/credit", data);
      // },
      generateSeed() {
        return Math.floor(Math.random() * 1000000000);
      },
      now() {
        return new Date().getTime();
      },
      api() {
        return api;
      }
    }
  }
  static version = ""
}


