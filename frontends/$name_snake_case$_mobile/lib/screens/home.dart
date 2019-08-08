import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:$name_snake_case$_mobile/blocs/blocs.dart';
import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_bloc.dart';
import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_event.dart';
import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_state.dart';
import 'package:$name_snake_case$_mobile/core/core.dart';
import 'package:$name_snake_case$_mobile/models/models.dart';
import 'package:$name_snake_case$_mobile/widgets/widgets.dart';

@immutable
class HomeScreen extends StatelessWidget {
  final String title;
  final $name_camel_case$Bloc $name_snake_case$Bloc;

  HomeScreen({Key key, this.title, this.$name_snake_case$Bloc}) : super(key: key) {
    print("init HomeScreen()");
  }

  @override
  Widget build(BuildContext context) {
    // final $name_snake_case$Bloc = BlocProvider.of<$name_camel_case$Bloc>(context);
    final tabBloc = BlocProvider.of<TabBloc>(context);

    return BlocBuilder<TabBloc, AppTab>(
      builder: (context, activeTab) {
        Widget body;
        if (activeTab == AppTab.timeline) {
          body = Timeline();
        } else if (activeTab == AppTab.notif) {
          body = NotifList(context);
        } else {
          // @TODO(*): fix this
          body = Timeline();
        }
        return Scaffold(
          appBar: AppBar(
            title: Text(title),
            // actions: [
            //   FilterButton(visible: activeTab == AppTab.todos),
            //   ExtraActions(),
            // ],
          ),
          drawer: new Drawer(
            child: ListView(
              children: <Widget>[
                new DrawerHeader(child: new Text("$name_camel_case$ Header")),
                new ListTile(
                    title: new Text("Accounts"),
                    onTap: () {
                      Navigator.pop(context);
                      // Navigator.of(context).pushNamed($name_camel_case$Routes.taskMan);
                    }),
                new ListTile(title: new Text("Analytics"), onTap: () {}),
                new Divider(),
                new ListTile(title: new Text("Notification"), onTap: () {}),
                new ListTile(title: new Text("Profile"), onTap: () {}),
                new ListTile(title: new Text("Security"), onTap: () {}),
                new Divider(),
                new ListTile(
                    title: new Text("Logout"),
                    onTap: () {
                      Navigator.pop(context);
                      $name_snake_case$Bloc.dispatch(LoggedOut());
                      Navigator.pushReplacementNamed(context, $name_camel_case$Routes.login);
                    }),
              ],
            ),
          ),
          body: body,
          // floatingActionButton: activeTab == AppTab.timeline
          //     ? FloatingActionButton(
          //         key: $name_camel_case$Keys.updateStatusFab,
          //         onPressed: () {
          //           Navigator.pushNamed(context, $name_camel_case$Routes.updateStatus);
          //         },
          //         child: Icon(Icons.add),
          //         tooltip: "Add comment",
          //       )
          //     : null,
          bottomNavigationBar: TabSelector(
            activeTab: activeTab,
            onTabSelected: (tab) => tabBloc.dispatch(UpdateTab(tab)),
          ),
        );
      },
    );
  }


}
