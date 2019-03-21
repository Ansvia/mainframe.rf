import 'package:fluro/fluro.dart';
import 'package:flutter/material.dart';
import 'package:$name$/screens/splash/index.dart';
import 'package:$name$/screens/home/index.dart';
import 'package:$name$/screens/login/index.dart';

void main() => runApp(MyApp());

class MyApp extends StatelessWidget {

  final Router router = new Router();
  
  @override
  Widget build(BuildContext context) {

    router.define('/home', handler: new Handler(handlerFunc: (BuildContext context, Map<String, dynamic> params) {
      return new HomeScreen();
    }));

    router.define('/login', handler: new Handler(handlerFunc: (BuildContext context, Map<String, dynamic> params) {
      return new LoginScreen();
    }));

    return MaterialApp(
      title: 'Flutter App',
      theme: ThemeData(
        primarySwatch: Colors.lightBlue
      ),
      home: new SplashScreen(),
      onGenerateRoute: router.generator
    );
  }

}
