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
import 'dart:io';
import 'package:http/http.dart' as http;
import 'package:$name_snake_case$_mobile/core/error.dart';
import 'package:$name_snake_case$_mobile/$param.service_name_snake_case$_repository/$param.service_name_snake_case$_repository.dart';

class ApiClient {
  static final ApiClient _singleton = new ApiClient._internal();
  static $param.service_name_pascal_case$Repository $param.service_name_camel_case$Repository;

  ApiResource private() =>
      new ApiResource("http://localhost:9090/api", $param.service_name_camel_case$Repository);
  ApiResource public() =>
      new ApiResource("http://localhost:8080/api", $param.service_name_camel_case$Repository);

  factory ApiClient() => _singleton;

  ApiClient._internal();
}

typedef ErrorHandlerFunc = void Function(Object error);

class ApiResource {
  final String baseUrl;
  static String accessToken = "";
  final $param.service_name_pascal_case$Repository _$param.service_name_camel_case$Repository;
  static ErrorHandlerFunc errorHandler = (err){};

  ApiResource(this.baseUrl, this._$param.service_name_camel_case$Repository);

  Future<dynamic> post(String apiPath, {Map<String, dynamic> body}) async {
    try {
      return _post(apiPath, body: body).then((resp){
        print("[POST $apiPath] resp: ${resp.body}");
        return resp;
      });
    } catch (e) {
      ApiResource.errorHandler(e);
    }
  }

  Future<dynamic> _post(String apiPath, {Map<String, dynamic> body}) async {
    var client = new http.Client();
    await ensureAccessToken();
    try {
      print("wanna to dispatch api at ${this.baseUrl + apiPath}");
      return client.post(this.baseUrl + apiPath,
          headers: buildHeaders(), body: json.encode(body));
    } on SocketException catch (e) {
      print("SocketException. $e");
      throw $name_pascal_case$Exception(
          "Cannot connect to server, please check your internet connection");
    } finally {
      client.close();
    }
  }

  Future<dynamic> get(String apiPath) async {
    try {
      return _get(apiPath);
    } catch (e) {
      ApiResource.errorHandler(e);
    }
  }

  Future<dynamic> _get(String apiPath) async {
    var client = new http.Client();
    await ensureAccessToken();
    try {
      return client.get(this.baseUrl + apiPath, headers: buildHeaders()).then((resp){
        print("[GET $apiPath] resp: ${resp.body}");
        return resp;
      });
    } on SocketException catch (e) {
      print("SocketException. $e");
      throw $name_pascal_case$Exception(
          "Cannot connect to server, please check your internet connection");
    } finally {
      client.close();
    }
  }

  Map<String, String> buildHeaders() {
    return {'Content-type': 'application/json', 'X-Access-Token': accessToken};
  }

  Future<void> ensureAccessToken() async {
    if (accessToken == null || accessToken == "") {
      accessToken = await _$param.service_name_camel_case$Repository.getToken();
      print("loading get access token: $accessToken");
    }
  }
}

/// Handle error in Future or async function.
void handleError(Object e) {
  try {
    _handleError(e);
  } catch (e) {
    ApiResource.errorHandler(e);
  }
}

void _handleError(Object e) {
  if (e is SocketException) {
    print("SocketException. $e");
    throw $name_pascal_case$Exception(
        "Cannot connect to server, please check your internet connection");
  } else if (e is http.ClientException) {
    throw $name_pascal_case$Exception("Cannot contact server");
  } else {
    print("Unhandled exception during calling Rest API: $e, ${e.runtimeType}");
    throw e;
  }
}
