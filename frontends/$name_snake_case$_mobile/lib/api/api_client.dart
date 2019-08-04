import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:$name_snake_case$_mobile/user_repository/user_repository.dart';

class ApiClient {
  static final ApiClient _singleton = new ApiClient._internal();
  static UserRepository userRepository;

  ApiResource private() =>
      new ApiResource("http://localhost:9090/api", userRepository);
  ApiResource public() =>
      new ApiResource("http://localhost:8080/api", userRepository);

  factory ApiClient() => _singleton;

  ApiClient._internal();

}

class ApiResource {
  final String baseUrl;
  static String accessToken = "";
  final UserRepository _userRepository;

  ApiResource(this.baseUrl, this._userRepository);

  Future<dynamic> post(String apiPath, {Map<String, String> body}) async {
    var client = new http.Client();
    await ensureAccessToken();
    try {
      print("wanna to dispatch api at ${this.baseUrl + apiPath}");
      return client.post(this.baseUrl + apiPath,
          headers: buildHeaders(), body: json.encode(body));
    } finally {
      client.close();
    }
  }

  Future<dynamic> get(String apiPath) async {
    var client = new http.Client();
    await ensureAccessToken();
    try {
      return client.get(this.baseUrl + apiPath, headers: buildHeaders());
    } finally {
      client.close();
    }
  }

  Map<String, String> buildHeaders() {
    return {'Content-type': 'application/json', 'X-Access-Token': accessToken};
  }

  Future<void> ensureAccessToken() async {
    if (accessToken == "") {
      accessToken = await _userRepository.getToken();
      print("loading get access token: $accessToken");
    }
  }
}
