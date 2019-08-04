import 'dart:convert';
import './$name_snake_case$_api.dart';
import './session.dart';

class Auth {
  static Future<Session> doLogin(String email, String password) {
    return ApiClient().public().post("/auth/v1/authorize",
        body: {'email': email, 'password': password}).then((resp) {

      final data = json.decode(resp.body);

      if (data['code'] != 0) {
        throw new Exception(data["description"]);
      }
      final d = data['result'];
      return new Session(d['account_id'], d['token']);
    });
  }

  static Future<void> doLogout() {
    return ApiClient()
        .public()
        .post("/auth/v1/unauthorize").then((resp) {

      final data = json.decode(resp.body);
      Auth.checkValidResp(data);
    });
  }

  static checkValidResp(Map<String, dynamic> respData) {
    if (respData['code'] != 0) {
      throw new Exception(respData["description"]);
    }
  }
}
