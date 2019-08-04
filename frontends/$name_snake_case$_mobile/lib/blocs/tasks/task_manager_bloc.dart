import 'dart:async';
import 'package:bloc/bloc.dart';
import 'package:meta/meta.dart';
import 'package:$name_snake_case$_mobile/blocs/tasks/task_manager_event.dart';
import 'package:$name_snake_case$_mobile/blocs/tasks/task_manager_state.dart';
import 'package:$name_snake_case$_mobile/models/task.dart';

class TaskManagerBloc extends Bloc<TaskManagerEvent, TaskManagerState> {
  TaskManagerBloc();
  
  @override
  TaskManagerState get initialState => TaskManagerUninitialized();
  
  @override
  Stream<TaskManagerState> mapEventToState(TaskManagerEvent event) async* {
     if (event is LoadTasks){
       yield* _mapLoadTasksToState();
     }
  }

  Stream<TaskManagerState> _mapLoadTasksToState() async* {
    // @TODO(*): code here
    yield TaskManagerListLoaded([
      Task(1, "Anto Suradiyanto", "Membuat laporan buat client BCA", "1 day left"),
      Task(2, "Gondez", "Analisa proyek PD", "3 days left")
    ]);
  }
}

