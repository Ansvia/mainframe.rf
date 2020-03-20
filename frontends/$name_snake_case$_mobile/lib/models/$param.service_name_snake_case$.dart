import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';

@immutable
class $param.service_name_pascal_case$ extends Equatable {
  final int id;
  final String fullName;
  final String email;

  const $param.service_name_pascal_case$(this.id, this.fullName, this.email);

  Map<String,dynamic> toMap(){
    Map<String,dynamic> data;
    data['id'] = this.id;
    data['full_name'] = this.fullName;
    data['email'] = this.email;
    return data;
  }

  static $param.service_name_pascal_case$ fromMap(Map<String, dynamic> data){
    return $param.service_name_pascal_case$(data['id'], data['fullName'], data['email']);
  }

  @override
  List<Object> get props => [this.id, this.fullName, this.email];
}
