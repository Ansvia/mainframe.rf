import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:$name_snake_case$_mobile/models/models.dart';


@immutable
abstract class TaskManagerEvent extends Equatable {
  TaskManagerEvent([List props = const []]) : super(props);
}

class LoadTasks extends TaskManagerEvent {
  LoadTasks();
  @override
  String toString() => "LoadTasks";
}

