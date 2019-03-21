import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter/cupertino.dart';
import 'package:$name$/data/db_helper.dart';

class SplashScreen extends StatefulWidget {
  @override
  State<StatefulWidget> createState() {
    return new SplashScreenState();
  }

}

class SplashScreenState extends State<SplashScreen> with SingleTickerProviderStateMixin {

  final DatabaseHelper dbHelper = new DatabaseHelper();

  var _visible = true;
  AnimationController animationController;
  Animation<double> animation;

  startTime() async {
    var _duration = new Duration(seconds: 3);
    return new Timer(_duration, navigationPage);
  }

  void navigationPage() {
    dbHelper.isLoggedIn().then((bool status) {
      if(status) Navigator.pushNamed(context, '/home');
      else Navigator.pushNamed(context, '/login');
    });
  }

  @override
  void initState() {
    super.initState();
    animationController = new AnimationController(
        vsync: this, duration: new Duration(seconds: 2));
    animation =
    new CurvedAnimation(parent: animationController, curve: Curves.easeOut);

    animation.addListener(() => this.setState(() {}));
    animationController.forward();

    setState(() {
      _visible = !_visible;
    });
    startTime();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.blueAccent,
      body: Stack(
        fit: StackFit.expand,
        children: <Widget>[
//          new Column(
//            mainAxisAlignment: MainAxisAlignment.end,
//            mainAxisSize: MainAxisSize.min,
//            children: <Widget>[
//              Padding(
//                  padding: EdgeInsets.only(bottom: 30.0),
//                  child: new Image.asset(
//                    'assets/avatars/avatar-1.jpg',
//                    height: 25.0,
//                    fit: BoxFit.scaleDown
//                  )
//              )
//            ]),
          new Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: <Widget>[
              new Icon(Icons.monetization_on, color: Colors.white, size: animation.value * 150),
//              new Image.asset(
//                'assets/tick.png',
//                width: animation.value * 250,
//                height: animation.value * 250,
//              ),
            ],
          ),
        ],
      ),
    );
  }

}