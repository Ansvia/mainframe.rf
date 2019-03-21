import 'package:flutter/material.dart';
import 'dart:math';

abstract class ClickHandler {
  void onItemClick(dynamic dataID);
}

class RowBoxData {
  int id;
  String title;
  String subtitle;
  String image;
  RowBoxData({this.id, this.title, this.subtitle, this.image});
}

class DataListBuilder {
  List<RowBoxData> rowItemList = new List<RowBoxData>();

  DataListBuilder(Object data) {
    for(var r in data) {
      rowItemList.add(new RowBoxData(
        id: r.id,
        title: r.title,
        subtitle: r.subtitle,
        image: "",
      ));
    }
  }
}

class ListViewContent extends StatelessWidget {

  final Object listObjects;
  final ClickHandler listener;
  final List<Color> circleColors = [
    Colors.redAccent, Colors.blueAccent, Colors.blueGrey, Colors.purpleAccent,
    Colors.greenAccent, Colors.amberAccent, Colors.green, Colors.pinkAccent,
    Colors.red, Colors.cyan, Colors.orange
  ];

  ListViewContent({this.listObjects, this.listener});

  Color randomGenerator() {
    return circleColors[new Random().nextInt(circleColors.length)];
  }

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
    DataListBuilder dataListBuilder = new DataListBuilder(this.listObjects);

    return (new ListView(
      children: dataListBuilder.rowItemList.map((RowBoxData rowBoxData) {
        return new ListTile(
          contentPadding: const EdgeInsets.fromLTRB(15, 8, 15, 8),
          onTap: () {
            listener.onItemClick(rowBoxData.id);
          },
          leading: new Container(
            margin: new EdgeInsets.only(
                left: 10.0, top: 10.0, bottom: 10.0, right: 10.0),
            width: 60.0,
            height: 60.0,
            child: CircleAvatar(
              backgroundColor: this.randomGenerator(),
              radius: 35.0,
              child: Text(
                capitalize(rowBoxData.title),
                style: TextStyle(
                  fontSize: 22.0,
                  color: Colors.white,
                ),
              ),
            ),
          ),
          title: new Text(
            rowBoxData.title,
            style: new TextStyle(fontSize: 18.0, fontWeight: FontWeight.w400),
          ),
          subtitle: new Text(
            rowBoxData.subtitle,
            style: new TextStyle(
                color: Colors.grey,
                fontSize: 14.0,
                fontWeight: FontWeight.w300)
          )
        );
     }).toList()
    ));
  }
}