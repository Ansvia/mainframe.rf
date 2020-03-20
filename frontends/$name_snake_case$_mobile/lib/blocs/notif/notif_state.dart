import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:$name_snake_case$_mobile/models/models.dart';

@immutable
abstract class NotifState extends Equatable {
  NotifState([List props = const []]) : super(props);
}

class NotifListLoading extends NotifState {
  @override
  String toString() => "NotifLoading";
}

class NotifListLoaded extends NotifState {
  final List<NotifItem> notifs;

  NotifListLoaded(this.notifs) : super([notifs]);

  @override
  String toString() => "NotifListLoaded";
}
