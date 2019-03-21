import 'package:flutter/material.dart';
import 'dart:math';

class ListData extends StatelessWidget {
  final EdgeInsets margin;
  final double width;
  final String title;
  final String subtitle;
  final List<Color> circleColors = [
    Colors.redAccent, Colors.blueAccent, Colors.blueGrey, Colors.purpleAccent,
    Colors.greenAccent, Colors.amberAccent, Colors.green, Colors.pinkAccent,
    Colors.red, Colors.cyan, Colors.orange
  ];

  Color randomGenerator() {
    return circleColors[new Random().nextInt(circleColors.length)];
  }
  ListData({this.margin, this.subtitle, this.title, this.width});


  String capitalize(String input) {
    if (input == null) {
      throw new ArgumentError("string: $input");
    }
    if (input.length == 0) {
      return input;
    }
    return input[0].toUpperCase();
  }

  @override
  Widget build(BuildContext context) {
    return (new Container(
      alignment: Alignment.center,
      margin: margin,
      width: width,
      decoration: new BoxDecoration(
        color: Colors.white,
        border: new Border(
//          top: new BorderSide(
//              width: 1.0, color: const Color.fromRGBO(204, 204, 204, 0.3)),
          bottom: new BorderSide(
              width: 1.0, color: const Color.fromRGBO(204, 204, 204, 0.3)),
        ),
      ),
      child: new Row(
        children: <Widget>[
          new Container(
              margin: new EdgeInsets.only(
                  left: 20.0, top: 10.0, bottom: 10.0, right: 20.0),
              width: 60.0,
              height: 60.0,
              child: CircleAvatar(
                backgroundColor: this.randomGenerator(),
                radius: 35.0,
                child: Text(
                  capitalize(title    ),
                  style: TextStyle(
                    fontSize: 22.0,
                    color: Colors.white,
                  ),
                ),
              ),
          ),
          new Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: <Widget>[
              new Text(
                title,
                style:
                    new TextStyle(fontSize: 18.0, fontWeight: FontWeight.w400),
              ),
              new Padding(
                padding: new EdgeInsets.only(top: 5.0),
                child: new Text(
                  subtitle,
                  style: new TextStyle(
                      color: Colors.grey,
                      fontSize: 14.0,
                      fontWeight: FontWeight.w300),
                ),
              )
            ],
          )
        ],
      ),
    ));
  }
}
