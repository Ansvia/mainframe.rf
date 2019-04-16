class Login {

  String _token;
  int _$param.service_name_snake_case$_id;
  Login(this._token, this._$param.service_name_snake_case$_id);

  Login.map(dynamic obj) {
    this._token = obj["token"];
    this._$param.service_name_snake_case$_id = obj["$param.service_name_snake_case$_id"];
  }

  String get token => _token;
  int get $param.service_name_snake_case$_id => _$param.service_name_snake_case$_id;

  Map<String, dynamic> toMap() {
    var map = new Map<String, dynamic>();
    map["token"] = _token;
    map["$param.service_name_snake_case$_id"] = _$param.service_name_snake_case$_id;

    return map;
  }
}