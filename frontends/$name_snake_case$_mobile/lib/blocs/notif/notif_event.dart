import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';

@immutable
abstract class NotifEvent extends Equatable {
  const NotifEvent();

  @override
  List<Object> get props => [];
}

class LoadNotif extends NotifEvent {
  LoadNotif();
  @override
  String toString() => "LoadNotif";
}
