import 'dart:io';
import 'package:$name$/utils/network_util.dart';
import 'package:$name$/utils/env.dart';

class ApiClient {

  publicApi(String token) {
    return new NetworkUtil(baseurl: publicURL, token: token);
  }

  privateApi(String token) {
    return new NetworkUtil(baseurl: privateURL, token: token);
  }

}