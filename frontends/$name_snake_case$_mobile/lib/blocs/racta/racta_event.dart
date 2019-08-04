
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:$name_snake_case$_mobile/models/models.dart';


@immutable
abstract class $name_camel_case$Event extends Equatable {
  $name_camel_case$Event([List props = const []]) : super(props);
}

class StartupEvent extends $name_camel_case$Event {
  StartupEvent();
}

class LoginInfo {
  String email;
  String password;

  LoginInfo(this.email, this.password);
}

// class LoginEvent extends $name_camel_case$Event {
//   final LoginInfo loginInfo;

//   LoginEvent(this.loginInfo) : super([loginInfo]);

//   @override
//   String toString() => "LoginEvent { loginInfo: $loginInfo }";
// }

class DeleteFeedItem extends $name_camel_case$Event {
  final FeedItem item;
  
  DeleteFeedItem(this.item);
  @override
  String toString() => "DeleteFeedItem";
}

class AddComment extends $name_camel_case$Event {
  final String text;
  AddComment(this.text);
  @override
  String toString() => "AddComment";
}

class LoggedIn extends $name_camel_case$Event {
  final String token;

  LoggedIn({@required this.token}) : super([token]);

  @override
  String toString() => 'LoggedIn { token: $token }';
}

class LoggedOut extends $name_camel_case$Event {
  @override
  String toString() => 'LoggedOut';
}
