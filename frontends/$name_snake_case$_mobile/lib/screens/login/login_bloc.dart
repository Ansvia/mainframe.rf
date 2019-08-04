import 'dart:async';

import 'package:meta/meta.dart';
import 'package:bloc/bloc.dart';
import 'package:$name_snake_case$_mobile/user_repository/user_repository.dart';
import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_bloc.dart';
import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_event.dart';
import 'package:$name_snake_case$_mobile/screens/login/login.dart';

class LoginBloc extends Bloc<LoginEvent, LoginState> {
  final UserRepository userRepository;
  final $name_camel_case$Bloc $name_snake_case$Bloc;

  LoginBloc({
    @required this.userRepository,
    @required this.$name_snake_case$Bloc,
  })  : assert(userRepository != null),
        assert($name_snake_case$Bloc != null);

  @override
  LoginState get initialState => LoginInitial();

  @override
  Stream<LoginState> mapEventToState(LoginEvent event) async* {
    if (event is LoginButtonPressed) {
      yield LoginLoading();

      try {
        final session = await userRepository.authenticate(
          email: event.email,
          password: event.password,
        );
        print("session: $session");
        $name_snake_case$Bloc.dispatch(LoggedIn(token: session.token));
        yield LoginInitial();
      } catch (error) {
        yield LoginFailure(error: error.toString());
      }
    }
  }
}
