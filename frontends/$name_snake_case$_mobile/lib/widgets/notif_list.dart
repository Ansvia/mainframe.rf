import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:$name_snake_case$_mobile/core/core.dart';
import 'package:$name_snake_case$_mobile/blocs/notif/notif.dart';
import 'package:$name_snake_case$_mobile/widgets/notif_item_view.dart';
import 'package:$name_snake_case$_mobile/widgets/widgets.dart';
import 'package:$name_snake_case$_mobile/widgets/loading_indicator.dart';

class NotifList extends StatelessWidget {

  NotifList(BuildContext context){
    // final notifBloc = BlocProvider.of<NotifBloc>(context);
    // notifBloc.dispatch(LoadNotif());
  }

  @override
  Widget build(BuildContext context) {
    // final $name_snake_case$Bloc = BlocProvider.of<$name_pascal_case$Bloc>(context);

    return BlocBuilder<NotifBloc, NotifState>(
      builder: (context, state) {
        if (state is NotifListLoading) {
          return LoadingIndicator(key: $name_pascal_case$Keys.loading);
        } else if (state is NotifListLoaded) {
          // return Text("satu");
          final notifs = state.notifs;
          return ListView.builder(
            key: $name_pascal_case$Keys.notifList,
            itemCount: notifs.length,
            itemBuilder: (BuildContext context, int index) {
              final item = notifs[index];
              return new NotifItemView(item: item);
            },
          );
        } else {
          return Text("Unknown state");
        }
      },
    );
  }
}
