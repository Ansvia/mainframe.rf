import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:$name_snake_case$_mobile/models/task.dart';


@immutable
abstract class TaskManagerState extends Equatable {
  TaskManagerState([List props = const []]) : super(props);
}

class TaskManagerUninitialized extends TaskManagerState {
  @override
  String toString() => "TaskManagerUninitialized";
}

class TaskManagerLoading extends TaskManagerState {
  @override
  String toString() => "TaskManagerLoading";
}

class TaskManagerListLoaded extends TaskManagerState {
  final List<Task> tasks;

  TaskManagerListLoaded(this.tasks): super([tasks]);
  
  @override
  String toString() => "TaskManagerListLoaded";
}

