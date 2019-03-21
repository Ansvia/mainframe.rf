import 'dart:async';
import 'dart:convert';
import 'package:$name$/data/api_client.dart';
import 'package:$name$/data/db_helper.dart';
import 'package:$name$/utils/crypt.dart';

class RestDatasource {

  final ApiClient api = new ApiClient();
  final Crypt crypt = new Crypt();
  final DatabaseHelper dbHelper = new DatabaseHelper();
  final JsonDecoder _decoder = new JsonDecoder();

  Future<dynamic> userInfo() async {
    return await dbHelper.getUserInfo().then((List user) {
      for(var r in user) {
        return api.publicApi(r['token']).get("/me/info").then((String result) {
          return _decoder.convert(result);
        });
      }
    });
  }

  unauthorize() async {
    await dbHelper.getUserInfo().then((List user) {
      user.forEach((r) async {
        api.publicApi('').post("/unauthorize", body: {});
        await dbHelper.deleteUsers();
      });
    });
  }

}