import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:$name_snake_case$_mobile/models/models.dart';
import 'package:$name_snake_case$_mobile/api/$name_snake_case$_api.dart';

@immutable
abstract class $name_pascal_case$State extends Equatable {
  const $name_pascal_case$State();
  @override
  List<Object> get props => [];
}

class $name_pascal_case$Loading extends $name_pascal_case$State {
  @override
  String toString() => "$name_pascal_case$Loading";
}

// class NotifListLoading extends $name_pascal_case$State {
//   @override
//   String toString() => "NotifListLoading";
// }

class AuthenticationUninitialized extends $name_pascal_case$State {
  @override
  String toString() => 'AuthenticationUninitialized';
}

class AuthenticationAuthenticated extends $name_pascal_case$State {
  @override
  String toString() => 'AuthenticationAuthenticated';
}

class AuthenticationUnauthenticated extends $name_pascal_case$State {
  @override
  String toString() => 'AuthenticationUnauthenticated';
}

class AuthenticationLoading extends $name_pascal_case$State {
  @override
  String toString() => 'AuthenticationLoading';
}


class LoginFailed extends $name_pascal_case$State {
  @override
  String toString() => "LoginFailed";
}

class LoginSuccess extends $name_pascal_case$State {
  final Session session;
  
  LoginSuccess(this.session);

  @override
  String toString() => "LoginSuccess { session: $session }";
}

class TimelineLoading extends $name_pascal_case$State {
  @override
  String toString() => "TimelineLoading";
}
