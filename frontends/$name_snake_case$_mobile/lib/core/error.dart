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

/// Error codes
class ErrorCode {
  static const int Unauthorized = 3000;
  static const int NotFound = 6002;
}

/// $name_pascal_case$ main exception type
class $name_pascal_case$Exception implements Exception {
  String message;
  final int code;

  $name_pascal_case$Exception(this.message, {this.code: 5001});

  /// Create [$name_pascal_case$Exception] instance from 
  /// server response error json.
  /// 
  /// Example:
  /// 
  /// ```
  /// $name_pascal_case$Exception.fromResp(decodedApiResp).withMsg("Cannot get user data");
  /// ```
  static $name_pascal_case$Exception fromResp(Map<String, dynamic> data){
    assert(data.containsKey('description'), "invalid error data, no `description` field");
    assert(data.containsKey('code'), "invalid error data, no `code` field");
    return $name_pascal_case$Exception(data['description'], code: data['code']);
  }

  /// Set custom error messag
  $name_pascal_case$Exception withMsg(String msg){
    this.message = msg;
    return this;
  }

  String toString(){
    return this.message;
  }
}



