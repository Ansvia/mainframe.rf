import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:$name_snake_case$_mobile/models/models.dart';
import 'package:$name_snake_case$_mobile/api/$name_snake_case$_api.dart';

@immutable
abstract class $name_camel_case$State extends Equatable {
  $name_camel_case$State([List props = const []]): super(props);
}

class $name_camel_case$Loading extends $name_camel_case$State {
  @override
  String toString() => "$name_camel_case$Loading";
}

// class NotifListLoading extends $name_camel_case$State {
//   @override
//   String toString() => "NotifListLoading";
// }

class AuthenticationUninitialized extends $name_camel_case$State {
  @override
  String toString() => 'AuthenticationUninitialized';
}

class AuthenticationAuthenticated extends $name_camel_case$State {
  @override
  String toString() => 'AuthenticationAuthenticated';
}

class AuthenticationUnauthenticated extends $name_camel_case$State {
  @override
  String toString() => 'AuthenticationUnauthenticated';
}

class AuthenticationLoading extends $name_camel_case$State {
  @override
  String toString() => 'AuthenticationLoading';
}


class LoginFailed extends $name_camel_case$State {
  @override
  String toString() => "LoginFailed";
}

class LoginSuccess extends $name_camel_case$State {
  final Session session;
  
  LoginSuccess(this.session);

  @override
  String toString() => "LoginSuccess { session: $session }";
}

class TimelineLoading extends $name_camel_case$State {
  @override
  String toString() => "TimelineLoading";
}

class TimelineLoaded extends $name_camel_case$State {
  final List<FeedItem> feeds;

  TimelineLoaded(this.feeds): super([feeds]);

  @override
  String toString() => "TimelineLoaded";
}

