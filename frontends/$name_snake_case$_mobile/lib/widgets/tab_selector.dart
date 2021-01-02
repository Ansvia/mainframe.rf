import 'package:flutter/cupertino.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:$name_snake_case$_mobile/core/core.dart';
import 'package:$name_snake_case$_mobile/models/models.dart';

class TabSelector extends StatelessWidget {
  final AppTab activeTab;
  final Function(AppTab) onTabSelected;

  TabSelector({
    Key key,
    @required this.activeTab,
    @required this.onTabSelected,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return BottomNavigationBar(
      key: $name_pascal_case$Keys.tabs,
      currentIndex: AppTab.values.indexOf(activeTab),
      onTap: (index) => onTabSelected(AppTab.values[index]),
      items: AppTab.values.map((tab) {
        IconData icon;
        Key key;
        String title;
        if (tab == AppTab.timeline) {
          icon = Icons.rss_feed;
          key = $name_pascal_case$Keys.timelineTab;
          title = "Timeline";
        } else if (tab == AppTab.notif) {
          icon = Icons.notifications;
          key = $name_pascal_case$Keys.notifTab;
          title = "Notif";
        } else if (tab == AppTab.todo) {
          icon = Icons.list;
          key = $name_pascal_case$Keys.todoTab;
          title = "Todo";
        } else {
          icon = Icons.dashboard;
          key = $name_pascal_case$Keys.dashboardTab;
          title = "Menu";
        }
        return BottomNavigationBarItem(
          icon: Icon(
            icon,
            key: key,
          ),
          label: title,
        );
      }).toList(),
    );
  }
}
