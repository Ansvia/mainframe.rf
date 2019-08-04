import 'package:flutter/material.dart';
import 'package:bloc/bloc.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:$name_snake_case$_mobile/api/api_client.dart';
import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_bloc.dart';
import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_event.dart';
import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_state.dart';
import 'package:$name_snake_case$_mobile/blocs/simple_bloc_delegate.dart';
import 'package:$name_snake_case$_mobile/blocs/tab/tab_bloc.dart';
import 'package:$name_snake_case$_mobile/blocs/tasks/tasks.dart';
import 'package:$name_snake_case$_mobile/screens/home.dart';
import 'package:$name_snake_case$_mobile/screens/login/login.dart';
import 'package:$name_snake_case$_mobile/screens/splash/splash_page.dart';
import 'package:$name_snake_case$_mobile/screens/tasks/tasks.dart';
import 'package:$name_snake_case$_mobile/user_repository/user_repository.dart';
import 'blocs/notif/notif.dart';

void main() {
  BlocSupervisor.delegate = SimpleBlocDelegate();
  
  final UserRepository userRepository = UserRepository();

  ApiClient.userRepository = userRepository;

  runApp(BlocProvider(
    builder: (ctx) {
      return $name_camel_case$Bloc(userRepository: userRepository)
        ..dispatch(StartupEvent());
    },
    child: $name_camel_case$App(userRepository: userRepository),
  ));

  // runApp(
  //   MultiBlocProvider(
  //     providers: [
  //       BlocProvider<$name_camel_case$Bloc>(
  //         builder: (context) =>
  //             $name_camel_case$Bloc(userRepository: userRepository),
  //       ),
  //       BlocProvider<AuthenticationBloc>(
  //           builder: (context) =>
  //               AuthenticationBloc(userRepository: userRepository)..dispatch(AppStarted())),
  //     ],
  //     child: $name_camel_case$App(userRepository: userRepository,),
  //   )
  // );
}

class $name_camel_case$App extends StatelessWidget {
  final UserRepository userRepository;

  $name_camel_case$App({Key key, @required this.userRepository}) : super(key: key);

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    final $name_snake_case$Bloc = BlocProvider.of<$name_camel_case$Bloc>(context);

    return MaterialApp(title: '$name$', theme: $name_camel_case$Theme.theme, routes: {
      "/": (context) {
        return BlocListener<$name_camel_case$Bloc, $name_camel_case$State>(
          listener: (BuildContext context, $name_camel_case$State state) {
            print("main state: $state");
            if (state is AuthenticationUnauthenticated) {
              Navigator.of(context).pushReplacementNamed('/login');
            } else if (state is AuthenticationAuthenticated) {
              Navigator.of(context).pushReplacementNamed('/inner');
            }
          },
          child: SplashPage(),
        );
      },
      "/inner": (context) {
        return MultiBlocProvider(
          providers: [
            BlocProvider<$name_camel_case$Bloc>(
              builder: (context) => $name_camel_case$Bloc(userRepository: userRepository),
            ),
            BlocProvider<TabBloc>(
              builder: (context) => TabBloc(),
            ),
            BlocProvider<NotifBloc>(builder: (context) => NotifBloc($name_snake_case$Bloc: $name_snake_case$Bloc),),
            // BlocProvider<TaskManagerBloc>(builder: (context) => TaskManagerBloc(),),
          ],
          child: HomeScreen(
            title: "$name$ Home",
            $name_snake_case$Bloc: $name_snake_case$Bloc
          ),
        );
      },
      "/login": (context) {
        return BlocListener<$name_camel_case$Bloc, $name_camel_case$State>(
          listener: (BuildContext context, $name_camel_case$State state) {
            if (state is AuthenticationAuthenticated) {
              Navigator.of(context).pushReplacementNamed('/inner');
            }
          },
          child: LoginPage(
            userRepository: userRepository,
          ),
        );

        // return MultiBlocProvider(
        //   providers: [
        //     BlocProvider<$name_camel_case$Bloc>(
        //       builder: (context) => $name_camel_case$Bloc(userRepository: userRepository),
        //     )
        //   ],
        //   child: LoginPage(
        //     userRepository: userRepository,
        //   ),
        // );
      },
      "/taskman": (context) {
        return BlocProvider(
          builder: (context){
            return TaskManagerBloc();
          },
          child: TaskManager(),
        );
      }
    });
  }
}


class $name_camel_case$Theme {
  static get theme {
    final originalTextTheme = ThemeData.dark().textTheme;
    final originalBody1 = originalTextTheme.body1;

    return ThemeData.dark().copyWith(
        primaryColor: Colors.grey[800],
        accentColor: Colors.cyan[300],
        buttonColor: Colors.grey[800],
        textSelectionColor: Colors.cyan[100],
        backgroundColor: Colors.grey[800],
        toggleableActiveColor: Colors.cyan[300],
        textTheme: originalTextTheme.copyWith(
            body1:
                originalBody1.copyWith(decorationColor: Colors.transparent)));
  }
}
