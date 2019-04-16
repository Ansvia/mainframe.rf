import 'dart:io';
import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:$name$/data/api_client.dart';
import 'package:$name$/data/res_api.dart';
import 'package:$name$/utils/auth.dart';
import 'package:$name$/screens/home/fragments/frag_dashboard.dart';
import 'package:$name$/screens/home/fragments/frag_$param.service_name_snake_case$s.dart';

class DrawerItem {
  String title;
  IconData icon;
  DrawerItem(this.title, this.icon);
}

class HomeScreen extends StatefulWidget {
  final drawerItems = [
    new DrawerItem("Dashboard", Icons.home),
    new DrawerItem("$param.service_name_camel_case$s", Icons.$param.service_name_snake_case$_circle),
    new DrawerItem("Logout", Icons.exit_to_app)
  ];

  @override
  HomeScreenState createState() => new HomeScreenState();
}

class HomeScreenState extends State<HomeScreen> {

  int _selectedDrawerIndex = 0;
  BuildContext _ctx;
  final RestDatasource apf = new RestDatasource();
  final ApiClient api = new ApiClient();
  final AuthStateProvider authStateProvider = new AuthStateProvider();
  final JsonDecoder _decoder = new JsonDecoder();

  _getDrawerItemWidget(int pos) {
    switch (pos) {
      case 0:
        return new DashboardFragment();
      case 1:
        return new $param.service_name_camel_case$sFragment(_ctx);
      case 2:
        return new Center(
          child: new RaisedButton(
            onPressed: () {
              _logout();
            },
            child: new Text("Logout")
          )
        );
      default:
        return new Text("Error");
    }
  }

  _onSelectItem(int index) {
    setState(() => _selectedDrawerIndex = index);
    Navigator.of(context).pop(); // close the drawer
  }

  @override
  void initState() {
    super.initState();
  }

  @override
  void dispose() {
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    _ctx = context;
    var drawerOptions = <Widget>[];
    for (var i = 0; i < widget.drawerItems.length; i++) {
      var d = widget.drawerItems[i];
      drawerOptions.add(
          new ListTile(
            leading: new Icon(d.icon),
            title: new Text(d.title),
            selected: i == _selectedDrawerIndex,
            onTap: () => _onSelectItem(i),
          )
      );
    }

    Widget drawerHeader = new FutureBuilder(
      future: apf.userInfo(),
      builder: (BuildContext context, AsyncSnapshot<dynamic> snapshot) {
        var decoded = snapshot.data;
        return new Column(
          children: <Widget>[
            new User$param.service_name_camel_case$sDrawerHeader(
              $param.service_name_snake_case$Name: new Text(
                snapshot.hasData ? decoded['full_name'] : "",
                style: TextStyle(
                    fontWeight: FontWeight.w700,
                    fontSize: 16.0,
                    color: Colors.white
                ),
              ),
              $param.service_name_snake_case$Email: new Text(
                snapshot.hasData ? decoded['email'] : "",
                style: TextStyle(
                  color: Colors.white,
                  fontStyle: FontStyle.italic
                ),
              ),
              current$param.service_name_camel_case$Picture: CircleAvatar(
                backgroundColor: Colors.white,
                child: Text(
                  snapshot.hasData ? _capitalize(decoded['full_name']) : "",
                  style: TextStyle(fontSize: 40.0),
                ),
              ),
            ),
            new Column(children: drawerOptions)
          ],
        );
      }
    );
    
    return (new WillPopScope(
        onWillPop: _onWillPop,
        child: new Scaffold(
          appBar: new AppBar(
            // here we display the title corresponding to the fragment
            // you can instead choose to have a static title
            title: new Text(
                widget.drawerItems[_selectedDrawerIndex].title,
              style: new TextStyle(
                color: Colors.white
              ),
            ),
          ),
          drawer: new Drawer(
            child: drawerHeader
          ),
          body: _getDrawerItemWidget(_selectedDrawerIndex),
        )
    ));


  }

  Future<bool> _onWillPop() {
    return showDialog(
      context: _ctx,
      child: new AlertDialog(
        title: new Text("Exit Application?",
            style: new TextStyle(
              fontSize: 20.0,
              fontWeight: FontWeight.w300,
              letterSpacing: 0.3,
            )),
        actions: <Widget>[
          new FlatButton(
            onPressed: () => Navigator.of(_ctx).pop(false),
            child: new Text('No'),
          ),
          new FlatButton(
            onPressed: () => exit(0),
            child: new Text('Yes'),
          )
        ],
      ),
    ) ?? false;
  }

  _logout() {
    apf.unauthorize();
    Navigator.pushNamed(context, '/login');
  }

  String _capitalize(String input) {
    if (input == null) {
      throw new ArgumentError("string: $input");
    }
    if (input.length == 0) {
      return input;
    }
    return input[0].toUpperCase();
  }

}
