// Copyright (C) 2016 Muqorrobien Ma'rufi
// All Rights Reserved.
//
// NOTICE: All information contained herein is, and remains
// the property of Muqorrobien Ma'rufi.
// The intellectual and technical concepts contained
// herein are proprietary to Muqorrobien Ma'rufi
// and are protected by trade secret or copyright law.
// Dissemination of this information or reproduction of this material
// is strictly forbidden unless prior written permission is obtained
// from Muqorrobien Ma'rufi (obin.mf@gmail.com).
//

import 'package:$name_snake_case$_mobile/core/error.dart';
import 'package:$name_snake_case$_mobile/models/$param.service_name_snake_case$.dart';
import 'package:$name_snake_case$_mobile/util/json_helper.dart';

import './$name_snake_case$_api.dart';
import './session.dart';

class Auth {
  static Future<Session> doLogin(String email, String password) {
    return ApiClient().public().post("/auth/v1/authorize",
        body: {'email': email, 'password': password}).then((resp) {
      // print("resp: $resp");
      final data = tryDecode(resp.body);
      print("login data: $data.body");
      if (data['code'] != 0) {
        switch (data['code']) {
          case ErrorCode.Unauthorized:
          case ErrorCode.NotFound:
            throw $name_pascal_case$Exception.fromResp(data)
                .withMsg("Wrong email or password");
            break;
          default:
            throw $name_pascal_case$Exception.fromResp(data);
        }
      }
      final d = data['result'];
      return new Session(d['account_id'], d['token']);
    }).catchError(handleError);
  }

  // load account information
  static Future<$param.service_name_pascal_case$> getMeInfo() {
    print("loading $param.service_name$ information...");
    return ApiClient().public().get("/$param.service_name_snake_case$/v1/me/info").then((resp) {
      print("resp: $resp");
      final data = tryDecode(resp.body);
      print("login data: $data");
      if (data['code'] != 0) {
        switch (data['code']) {
          case ErrorCode.Unauthorized:
          case ErrorCode.NotFound:
            throw $name_pascal_case$Exception.fromResp(data).withMsg("$param.service_name_pascal_case$ not found");
            break;
          default:
            throw $name_pascal_case$Exception.fromResp(data);
        }
      }
      return $param.service_name_pascal_case$.fromMap(data["result"]);
    });
  }

  static Future<void> doLogout() {
    return ApiClient().public().post("/auth/v1/unauthorize").then((resp) {
      print("resp: ${resp.body}");
      final data = tryDecode(resp.body);
      checkValidResp(data);
    }).catchError(handleError, test: (_) => true);
  }
}
