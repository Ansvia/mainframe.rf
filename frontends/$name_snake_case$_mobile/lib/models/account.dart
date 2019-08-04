import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';

@immutable
class Account extends Equatable {
  final int id;
  final String fullName;
  final String email;

  Account(this.id, this.fullName, this.email)
      : super([id, fullName, email]);

  Map<String,dynamic> toMap(){
    Map<String,dynamic> data;
    data['id'] = this.id;
    data['fullName'] = this.fullName;
    data['email'] = this.email;
    return data;
  }

  static Account fromMap(Map<String, dynamic> data){
    return Account(data['id'], data['fullName'], data['email']);
  }
}
