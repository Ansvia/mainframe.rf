import 'package:flutter/material.dart';

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_bloc.dart';
import 'package:$name_snake_case$_mobile/$param.service_name_snake_case$_repository/$param.service_name_snake_case$_repository.dart';
import 'package:$name_snake_case$_mobile/screens/login/login.dart';

class LoginPage extends StatelessWidget {
  final $param.service_name_pascal_case$Repository $param.service_name_camel_case$Repository;

  LoginPage({Key key, @required this.$param.service_name_camel_case$Repository})
      : assert($param.service_name_camel_case$Repository != null),
        super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      // appBar: AppBar(
      //   title: Text('Login'),
      // ),
      body: BlocProvider(
        create: (context) {
          return LoginBloc(
            $name_camel_case$Bloc: BlocProvider.of<$name_pascal_case$Bloc>(context),
            $param.service_name_camel_case$Repository: $param.service_name_camel_case$Repository,
          );
        },
        child: LoginForm(),
      ),
    );
  }
}
