

import 'dart:convert';

import 'package:$name_snake_case$_mobile/core/error.dart';


/// Try to decode json text into [Map<String, dynamic>]
/// Will raise [$name_pascal_case$Exception] when something went wrong.
Map<String, dynamic> tryDecode(String txtData){
  try {
    var data = json.decode(txtData);
    return data;
  } catch (e) {
    print("Cannot decode json message: " + txtData);
    throw $name_pascal_case$Exception("Cannot communicate with server", code: 4001);
  }
}