import 'dart:convert';
import 'package:$name$/data/api_client.dart';
import 'package:$name$/utils/crypt.dart';
import 'package:$name$/utils/env.dart';
import 'package:$name$/models/login.dart';

abstract class LoginScreenContract {
  void onLoginSuccess(Login login);
  void onLoginError(String errorTxt);
}

class LoginScreenPresenter {
  LoginScreenContract _view;
  LoginScreenPresenter(this._view);
  final ApiClient api = new ApiClient();
  final Crypt crypt = new Crypt();
  final JsonDecoder _decoder = new JsonDecoder();

  doLogin(String username, String password) {
    print("in doLogin()");
    // <% if param.password_crypt_algo == "sha256" %>
    var resp = api.publicApi('').post('/auth/v1/authorize', body: {
      "email": username,
      "phone": '',
      "passhash": crypt.getPassHash(password)
    });
    // <% endif %>

    // <% if param.password_crypt_algo == "bcrypt" %>
    var resp = api.publicApi('').post('/auth/v1/authorize', body: {
      "email": username,
      "phone": '',
      "password": password
    });
    // <% endif %>

    resp.then((String response) {
      var res = _decoder.convert(response);
      if(res["code"] == 0) {
        print("[i] Login success!");
        _view.onLoginSuccess(new Login.map(res));
      } else {
        print("[i] Login failed!");
        _view.onLoginError("[i] Login failed!");
      }
    });
  }
}