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

import 'dart:convert';
import 'package:localstorage/localstorage.dart';
import 'package:$name_snake_case$_mobile/api/$name_snake_case$_api.dart';
import 'package:$name_snake_case$_mobile/db_helper.dart';

abstract class LayeredRepo {
  Future<T> fetch<T extends dynamic>(
      String storeName, Future<T> Function() dataRetriever,
      {force: bool});

  Future<Map<String, dynamic>> fetchApi(String storeName, String apiPath, {force: bool});

  void clear();
}

class LocalLayeredRepo extends LayeredRepo {
  final String key;
  final LocalStorage _storage;

  LocalLayeredRepo(this.key) : _storage = LocalStorage(key) {
    var appConfig = LocalStorage("__app_config__");
    if (appConfig.getItem("resetData") == true){
      this.clear();
    }
  }

  Future<T> fetch<T extends dynamic>(
      String storeName, Future<T> Function() dataRetriever,
      {force: bool}) async {
    T resultData = _storage.getItem(storeName);

    print("resultData: $resultData");

    if (resultData == null || force == true) {
      final data = await dataRetriever();
      if (data != null) {
        print("resp data: $data");
        resultData = data["result"];
      }
    }

    _storage.setItem(storeName, resultData);

    return resultData;
  }

  Future<Map<String, dynamic>> fetchApi(String storeName, String apiPath, {force: bool}){
    return fetch(storeName, () => PublicApi.get(apiPath), force: force);
  }

  void clear() {
    _storage.clear();
  }
}

class PersistentLayeredRepo extends LayeredRepo {
  final String key;

  PersistentLayeredRepo(this.key) {
    DatabaseHelper().db.then((db) {
      // db.execute('DROP TABLE $key');
      db.execute(
          "CREATE TABLE IF NOT EXISTS $key (t_key TEXT PRIMARY KEY, t_val TEXT)");
    });
  }

  Future<T> fetch<T extends dynamic>(
      String storeName, Future<T> Function() dataRetriever,
      {force: bool}) async {
    final dbClient = await DatabaseHelper().db;

    List<Map> result = await dbClient
        .rawQuery('SELECT * FROM $key WHERE t_key=\'$storeName\' LIMIT 1');

    T resultData;

    if (result.length > 0) {
      resultData = json.decode(result.first["t_val"]);
    } else {
      resultData = null;
    }

    print("resultData: $resultData");

    if (resultData == null || force == true) {
      final data = await dataRetriever();
      if (data != null) {
        print("resp data: $data");

        // await dbClient.insert(
        //     key, {"t_key": storeName, "t_val": json.encode(data["result"])});

        final tVal = json.encode(data["result"]);

        await dbClient.rawQuery(
            "INSERT OR REPLACE INTO $key (t_key, t_val)VALUES('$storeName', '$tVal')");

        resultData = data["result"];
      }
    }

    return resultData;
  }

  Future<Map<String, dynamic>> fetchApi(String storeName, String apiPath, {force: bool}){
    return fetch(storeName, () => PublicApi.get(apiPath), force: force);
  }

  /// Update entries ini akan me-replace data di dalam array
  /// dilakukan dengan cara pengecheckan berdasarkan id-nya.
  Future<void> updateEntriesItem(
      String storeName, Map<String, dynamic> value) async {
    final dbClient = await DatabaseHelper().db;

    List<Map> result = await dbClient
        .rawQuery('SELECT * FROM $key WHERE t_key=\'$storeName\' LIMIT 1');

    Map<String, dynamic> resultData;

    if (result.length > 0) {
      resultData = json.decode(result.first["t_val"]);
    } else {
      resultData = null;
    }

    print("updateEntriesItem > resultData: $resultData");

    if (resultData != null) {
      bool replaced = false;
      List<dynamic> newEntries = (resultData["entries"] as List).map((a) {
        if (a["id"] == value["id"]) {
          replaced = true;
          return value;
        } else {
          return a;
        }
      }).toList();

      // jika belum ada maka tambahkan
      if (!replaced) {
        newEntries.add(value);
      }

      // await dbClient.insert(
      //       key, {"t_key": storeName, "t_val": json.encode(newEntries)});
      final tVal = json.encode({"entries": newEntries});

      await dbClient.rawQuery(
          "INSERT OR REPLACE INTO $key (t_key, t_val)VALUES('$storeName', '$tVal')");
    }
  }

  void clear() async {
    final dbClient = await DatabaseHelper().db;
    dbClient.delete(key);
  }
}
