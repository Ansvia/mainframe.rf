
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:$name_snake_case$_mobile/models/models.dart';


@immutable
abstract class $name_pascal_case$Event extends Equatable {
  const $name_pascal_case$Event();

  @override
  List<Object> get props => [];
}

class StartupEvent extends $name_pascal_case$Event {
  StartupEvent();
}

class LoginInfo {
  String email;
  String password;

  LoginInfo(this.email, this.password);
}

class AddComment extends $name_pascal_case$Event {
  final String text;
  AddComment(this.text);
  @override
  String toString() => "AddComment";
}

class LoggedIn extends $name_pascal_case$Event {
  final String token;

  LoggedIn({@required this.token});

  @override
  List<Object> get props => [this.token];

  @override
  String toString() => 'LoggedIn { token: $token }';
}

class LoggedOut extends $name_pascal_case$Event {
  @override
  String toString() => 'LoggedOut';
}
