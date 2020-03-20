import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:$name_snake_case$_mobile/models/models.dart';

@immutable
abstract class NotifState extends Equatable {
  const NotifState();

  @override
  List<Object> get props => [];
}

class NotifListLoading extends NotifState {
  @override
  String toString() => "NotifLoading";
}

class NotifListLoaded extends NotifState {
  final List<NotifItem> notifs;

  NotifListLoaded(this.notifs);

  @override
  List<Object> get props => [this.notifs];

  @override
  String toString() => "NotifListLoaded";
}
