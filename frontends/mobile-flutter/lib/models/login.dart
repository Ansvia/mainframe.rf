class Login {

  String _token;
  int _account_id;
  Login(this._token, this._account_id);

  Login.map(dynamic obj) {
    this._token = obj["token"];
    this._account_id = obj["account_id"];
  }

  String get token => _token;
  int get account_id => _account_id;

  Map<String, dynamic> toMap() {
    var map = new Map<String, dynamic>();
    map["token"] = _token;
    map["account_id"] = _account_id;

    return map;
  }
}