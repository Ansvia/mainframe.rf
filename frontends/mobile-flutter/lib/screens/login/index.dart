import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:$name$/screens/login/styles.dart';
import 'package:$name$/utils/auth.dart';
import 'package:$name$/data/db_helper.dart';
import 'package:$name$/models/login.dart';
import 'package:$name$/Components/InputFields.dart';
import 'package:$name$/Components/WhiteTick.dart';
import 'package:$name$/screens/login/loginScreenPresenter.dart';

class LoginScreen extends StatefulWidget {
  @override
  State<StatefulWidget> createState() {
      LoginScreenState loginScreenState = new LoginScreenState();
      return loginScreenState;
  }
}

class LoginScreenState extends State<LoginScreen>
    implements LoginScreenContract, AuthStateListener {
  BuildContext _ctx;

  int _state = 0;
  final formKey = new GlobalKey<FormState>();
  final scaffoldKey = new GlobalKey<ScaffoldState>();
  String _password, _email;

  LoginScreenPresenter _presenter;

  LoginScreenState() {
    _presenter = new LoginScreenPresenter(this);
    var authStateProvider = new AuthStateProvider();
    authStateProvider.subscribe(this);
  }

  void _submit() async {
    final form = formKey.currentState;

    if (form.validate()) {
      setState(() => _state = 1);
      form.save();
      await _presenter.doLogin(_email, _password);
    }
  }

  void _showSnackBar(String text) {
    scaffoldKey.currentState
        .showSnackBar(new SnackBar(content: new Text(text)));
  }

  @override
  onAuthStateChanged(AuthState state) {
    if(state == AuthState.LOGGED_IN)
      Navigator.pushNamed(_ctx, '/home');
  }

  @override
  Widget build(BuildContext context) {
    _ctx = context;
    var loginBtn = new Container(
      width: double.infinity,
      height: 60.0,
      margin: const EdgeInsets.fromLTRB(18.0, 20.0, 18.0, 20.0),
      alignment: FractionalOffset.center,
      decoration: new BoxDecoration(
        color: const Color.fromRGBO(247, 64, 106, 1.0),
        borderRadius: new BorderRadius.all(const Radius.circular(30.0)),
      ),
      child: Padding(
        padding: EdgeInsets.symmetric(vertical: 0.0),
        child: Material(
          borderRadius: BorderRadius.circular(30.0),//Set this up for rounding corners.
          shadowColor: Colors.lightBlueAccent.shade100,
          child: MaterialButton(
            minWidth: double.infinity,
            height: 60.0,
            onPressed: () {
              if(_state == 0)
                _submit();
            },
            color: const Color.fromRGBO(247, 64, 106, 1.0),
            child: setupButtonChild(),
          ),
        ),
      ),
    );

    var loginForm = new Container(
      margin: new EdgeInsets.symmetric(horizontal: 20.0),
      child: new Column(
        mainAxisAlignment: MainAxisAlignment.spaceEvenly,
        children: <Widget>[
          new Form(
            key: formKey,
            child: new Column(
              mainAxisAlignment: MainAxisAlignment.spaceAround,
              children: <Widget>[
                new InputFieldArea(
                  hint: "Email / Phone Number",
                  obscure: false,
                  icon: Icons.person_outline,
                  onSaved: (val) => _email = val
                ),
                new InputFieldArea(
                  hint: "Password",
                  obscure: true,
                  icon: Icons.lock_outline,
                  onSaved: (val) => _password = val,
                ),
              ],
            ),
          )
        ],
      ),
    );

    return new Scaffold(
      appBar: null,
      key: scaffoldKey,
      body: new Container(
        decoration: new BoxDecoration(
          image: backgroundImage,
        ),
        child: new Container(
          decoration: new BoxDecoration(
            gradient: new LinearGradient(
              colors: <Color>[
                const Color.fromRGBO(162, 146, 199, 0.8),
                const Color.fromRGBO(51, 51, 63, 0.9),
              ],
              stops: [0.2, 1.0],
              begin: const FractionalOffset(0.0, 0.0),
              end: const FractionalOffset(0.0, 1.0),
            )
          ),
          child: new ListView(
            padding: const EdgeInsets.all(0.0),
            children: <Widget>[
              new Stack(
                alignment: AlignmentDirectional.bottomCenter,
                children: <Widget>[
                  new Column(
                    mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                    children: <Widget>[
                      new Tick(image: tick),
                      loginForm,
                      loginBtn
                    ],
                  )
                ],
              )
            ],
          ),
      ),
      )
    );
  }

  Widget setupButtonChild() {
    if (_state == 0) {
      return new Text("LOGIN",
        style: new TextStyle(
          color: Colors.white,
          fontSize: 20.0,
          fontWeight: FontWeight.w300,
          letterSpacing: 0.3,
        ),
      );
    } else if (_state == 1) {
      return CircularProgressIndicator(
        valueColor: AlwaysStoppedAnimation<Color>(Colors.white),
      );
    } else if (_state == 2) {
      return Icon(Icons.check_circle_outline, color: Colors.white);
    } else if (_state == 3) {
      return Icon(Icons.warning, color: Colors.white);
    }
  }

  _onWillPop(String title, String text) {
    return showDialog(
      context: _ctx,
      child: new AlertDialog(
        title: new Text(title),
        content: new Text(text),
        actions: <Widget>[
          new FlatButton(
            onPressed: () {
              setState(() => _state = 0);
              Navigator.of(_ctx).pop(false);
            },
            child: new Text('OK'),
          )
        ],
      ),
    );
  }

  @override
  void onLoginError(String errorTxt) {
    setState(() => _state = 3);
    _onWillPop("Login Failed!","Please check your email or password");
  }

  @override
  void onLoginSuccess(Login login) async {
    setState(() => _state = 2);
    var db = new DatabaseHelper();
    await db.saveUser(login);
    var authStateProvider = new AuthStateProvider();
    authStateProvider.notify(AuthState.LOGGED_IN);
  }
}