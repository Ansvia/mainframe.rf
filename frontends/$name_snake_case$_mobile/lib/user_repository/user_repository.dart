import 'dart:async';

import 'package:meta/meta.dart';
import 'package:$name_snake_case$_mobile/api/$name_snake_case$_api.dart';
import 'package:$name_snake_case$_mobile/db_helper.dart';

class UserRepository {
  Future<Session> authenticate({
    @required String email,
    @required String password,
  }) async {
    final session = await Auth.doLogin(email, password);
    DatabaseHelper().db.then((db) {
      db.insert("AccessToken", {'token': session.token});
    });
    return session;
  }

  Future<void> deleteToken() async {
    await Auth.doLogout();
    await DatabaseHelper().deleteUser();
    await DatabaseHelper().db.then((db) {
      db.delete('AccessToken');
    });
    return;
  }

  Future<void> persistToken(String token) async {
    var db = await DatabaseHelper().db;
    db.insert("AccessToken", {'token': token});
    return;
  }

  Future<bool> hasToken() async {
    var db = await DatabaseHelper().db;
    return db.query('AccessToken').then((res) => res.length > 0);
  }

  Future<String> getToken() async {
    return DatabaseHelper().db.then((db) => db.query('AccessToken').then((res) {
          if (res.length > 0) {
            return res.first['token'];
          } else {
            return '';
          }
        }));
  }
}
