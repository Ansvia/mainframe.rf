import 'dart:async';

import 'package:meta/meta.dart';
import 'package:$name_snake_case$_mobile/api/$name_snake_case$_api.dart';
import 'package:$name_snake_case$_mobile/core/smart_repo.dart';
import 'package:$name_snake_case$_mobile/models/models.dart';

class $param.service_name_pascal_case$Repository {
  PersistentSmartRepo repo;

  $param.service_name_pascal_case$Repository(){
    repo = new PersistentSmartRepo('$param.service_name_pascal_case$');
  }

  Future<Session> authenticate({
    @required String email,
    @required String password,
  }) async {
    return Auth.doLogin(email, password).then((session) async {
      if (session != null){
        repo.putData("accessToken", session.toMap());
      }else{
        throw $name_pascal_case$Exception(
          "Cannot contact to server");
      }
      return session;
    }).whenComplete(() async {
      repo.fetchGradually("current$param.service_name_pascal_case$", () => PublicApi.get("/$param.service_name_snake_case$/v1/me/info"), force: true);
    });
  }

  Future<void> deleteToken() async {
    await Auth.doLogout();
    repo.clear();
    return;
  }

  Future<void> persistToken(String token) async {
    repo.putData("accessToken", {"token": token});
    return;
  }

  Future<bool> hasToken() async {
    var token = await getToken();
    return token != null;
  }

  Future<$param.service_name_pascal_case$> get$param.service_name_pascal_case$Info() {
    return repo.fetchGradually("current$param.service_name_pascal_case$", () => PublicApi.get("/$param.service_name_snake_case$/v1/me/info"), force: true)
      .map((a) => $param.service_name_pascal_case$.fromMap(a.data))
      .first;
  }

  Future<String> getToken() async {
    return repo.getData("accessToken").then((data){
      print("get accessToken in getToken(): $data");
      if (data != null){
        return data["token"] as String;
      }else{
        return null;
      }
    });
  }
}
