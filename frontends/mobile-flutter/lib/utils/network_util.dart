import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:http/http.dart' as http;

class NetworkUtil {

  final String baseurl;
  final String token;

  NetworkUtil({this.baseurl, this.token});

  final client = new http.Client();

  Future<String> get(String path) async {
    final response = await http.get(
      baseurl + path,
      headers: {
        HttpHeaders.contentTypeHeader: 'application/json',
        'X-Access-Token': token
      }
    );
    return response.body;
  }

  Future<String> post(String path, {Map headers, body, encoding}) async {
    return await client.post(
      Uri.parse(baseurl + path),
      headers: {HttpHeaders.contentTypeHeader: 'application/json'}, 
      body: json.encode(body)
    ).then((http.Response response) {
      final String res = response.body;
      final int statusCode = response.statusCode;
      if (statusCode < 200 || statusCode > 400 || json == null) {
        return res;
      } else {
        return res;
      }
    }).whenComplete(client.close);
  }
}