import 'package:flutter/material.dart';

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:$name_snake_case$_mobile/core/$name_snake_case$_app_core.dart';
import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_bloc.dart';
import 'package:$name_snake_case$_mobile/blocs/tasks/tasks.dart';
import 'package:$name_snake_case$_mobile/widgets/loading_indicator.dart';
import 'package:$name_snake_case$_mobile/widgets/task_item_view.dart';

class TaskManager extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final $name_snake_case$Bloc = BlocProvider.of<$name_camel_case$Bloc>(context);

    return BlocBuilder<TaskManagerBloc, TaskManagerState>(
      builder: (context, state) {
        if (state is TaskManagerUninitialized) {
          BlocProvider.of<TaskManagerBloc>(context).dispatch(LoadTasks());
          return LoadingIndicator(key: $name_camel_case$Keys.loading);
        } else if (state is TaskManagerLoading) {
          return LoadingIndicator(key: $name_camel_case$Keys.loading);
        } else if (state is TaskManagerListLoaded) {
          // return Text("satu");
          final notifs = state.tasks;
          return Scaffold(
            appBar: AppBar(
              title: Text("Tasks"),
            ),
            body: ListView.builder(
              key: $name_camel_case$Keys.notifList,
              itemCount: notifs.length,
              itemBuilder: (BuildContext context, int index) {
                final item = notifs[index];
                // @TODO(*): code here
                return new TaskItemView(item: item, onTap: (){},);
              },
            ),
            floatingActionButton: FloatingActionButton(
              key: $name_camel_case$Keys.addTask,
              onPressed: () {
                Navigator.of(context).pushNamed("/task/add");
              },
              child: Icon(Icons.add),
              tooltip: "Create new task",
            ),
          );
        } else {
          return Text("Unknown state");
        }
      },
    );
  }
}
