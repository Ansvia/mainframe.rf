import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:$name_snake_case$_mobile/models/models.dart';

@immutable
abstract class TabEvent extends Equatable {
  const TabEvent();

  @override
  List<Object> get props => [];
}

class UpdateTab extends TabEvent {
  final AppTab tab;

  UpdateTab(this.tab);

  @override
  List<Object> get props => [this.tab];

  @override
  String toString() => 'UpdateTab { tab: $tab }';
}
