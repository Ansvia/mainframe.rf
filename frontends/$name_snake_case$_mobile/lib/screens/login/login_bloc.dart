import 'dart:async';

import 'package:meta/meta.dart';
import 'package:bloc/bloc.dart';
import 'package:$name_snake_case$_mobile/$param.service_name_snake_case$_repository/$param.service_name_snake_case$_repository.dart';
import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_bloc.dart';
import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_event.dart';
import 'package:$name_snake_case$_mobile/screens/login/login.dart';

class LoginBloc extends Bloc<LoginEvent, LoginState> {
  final $param.service_name_pascal_case$Repository $param.service_name_camel_case$Repository;
  final $name_pascal_case$Bloc $name_camel_case$Bloc;

  LoginBloc({
    @required this.$param.service_name_camel_case$Repository,
    @required this.$name_camel_case$Bloc,
  })  : assert($param.service_name_camel_case$Repository != null),
        assert($name_camel_case$Bloc != null);

  @override
  LoginState get initialState => LoginInitial();

  @override
  Stream<LoginState> mapEventToState(LoginEvent event) async* {
    if (event is LoginButtonPressed) {
      yield LoginLoading();

      try {
        final session = await $param.service_name_camel_case$Repository.authenticate(
          email: event.email,
          password: event.password,
        );
        print("session: $session");
        $name_camel_case$Bloc.add(LoggedIn(token: session.token));
        yield LoginInitial();
      } catch (error) {
        yield LoginFailure(error: error.toString());
      }
    }
  }
}
