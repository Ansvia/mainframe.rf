import 'dart:async';
import 'package:bloc/bloc.dart';
import 'package:meta/meta.dart';
import 'package:$name_snake_case$_mobile/api/api_client.dart';
import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_event.dart';
import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_state.dart';
import 'package:$name_snake_case$_mobile/$param.service_name_snake_case$_repository/$param.service_name_snake_case$_repository.dart';

class $name_pascal_case$Bloc extends Bloc<$name_pascal_case$Event, $name_pascal_case$State> {
  final $param.service_name_pascal_case$Repository $param.service_name_camel_case$Repository;
  $name_pascal_case$Bloc({@required this.$param.service_name_camel_case$Repository}) : assert($param.service_name_camel_case$Repository != null), super($name_pascal_case$Loading());

  @override
  Stream<$name_pascal_case$State> mapEventToState($name_pascal_case$Event event) async* {
    if (event is LoggedIn) {
      yield* _mapLogin$name_pascal_case$ToState(event);
    } else if (event is StartupEvent) {
      print("Got startup event");
      yield* _mapStartupToState(event);
    } else if (event is LoggedOut) {
      yield* _mapLoggedOutToState(event);
    }
  }

  Stream<$name_pascal_case$State> _mapLogin$name_pascal_case$ToState(LoggedIn event) async* {
    yield AuthenticationLoading();
    await $param.service_name_camel_case$Repository.persistToken(event.token);
    yield AuthenticationAuthenticated();
  }

  Stream<$name_pascal_case$State> _mapStartupToState(StartupEvent event) async* {
    final bool hasToken = await $param.service_name_camel_case$Repository.hasToken();

    if (hasToken) {
      yield AuthenticationAuthenticated();
    } else {
      yield AuthenticationUnauthenticated();
    }
  }

  Stream<$name_pascal_case$State> _mapLoggedOutToState(LoggedOut event) async* {
    yield AuthenticationLoading();
    await $param.service_name_camel_case$Repository.deleteToken();
    ApiResource.accessToken = "";
    yield AuthenticationUnauthenticated();
  }

}
