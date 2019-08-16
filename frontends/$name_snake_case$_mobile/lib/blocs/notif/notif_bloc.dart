import 'dart:async';
import 'package:bloc/bloc.dart';
import 'package:meta/meta.dart';
import 'package:$name_snake_case$_mobile/blocs/notif/notif_event.dart';
import 'package:$name_snake_case$_mobile/blocs/notif/notif_state.dart';
import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_bloc.dart';
import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_state.dart';
import 'package:$name_snake_case$_mobile/models/notif_item.dart';

class NotifBloc extends Bloc<NotifEvent, NotifState> {
  final $name_pascal_case$Bloc $name_snake_case$Bloc;
  StreamSubscription _$name_snake_case$Subs;

  NotifBloc({@required this.$name_snake_case$Bloc}){
    _$name_snake_case$Subs = $name_snake_case$Bloc.state.listen((state){
      if (state is AuthenticationAuthenticated){
        dispatch(LoadNotif());
      }
    });
  }


  @override
  NotifState get initialState => NotifListLoading();

  @override
  Stream<NotifState> mapEventToState(NotifEvent event) async* {
    if (event is LoadNotif) {
      yield* _mapLoadNotifToState(event);
    }
  }

  Stream<NotifState> _mapLoadNotifToState(LoadNotif event) async* {
    // @TODO(*): fix this
    yield NotifListLoading();
    // final result = ApiClient.public().
    yield NotifListLoaded([NotifItem(1, "a", "Halo", 1, 2, [], "1 hour ago")]);
  }

}
