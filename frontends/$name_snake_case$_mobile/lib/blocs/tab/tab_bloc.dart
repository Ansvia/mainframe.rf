import 'dart:async';
import 'package:bloc/bloc.dart';
import 'package:$name_snake_case$_mobile/blocs/tab/tab_event.dart';
import 'package:$name_snake_case$_mobile/models/app_tab.dart';

class TabBloc extends Bloc<TabEvent, AppTab> {
  TabBloc() : super(AppTab.timeline);

  @override
  Stream<AppTab> mapEventToState(TabEvent event) async* {
    if (event is UpdateTab) {
      yield event.tab;
    }
  }
}
