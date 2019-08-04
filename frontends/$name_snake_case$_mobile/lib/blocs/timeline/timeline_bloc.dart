// import 'dart:async';
// import 'package:bloc/bloc.dart';
// import 'package:meta/meta.dart';
// import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_bloc.dart';
// import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_event.dart';
// import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_state.dart';
// import 'package:$name_snake_case$_mobile/blocs/timeline/timeline_event.dart';
// import 'package:$name_snake_case$_mobile/blocs/timeline/timeline_state.dart';
// import 'package:$name_snake_case$_mobile/models/models.dart';

// class TimelineBloc extends Bloc<TimelineEvent, TimelineState> {
//   final $name_camel_case$Bloc $name_snake_case$Bloc;
//   StreamSubscription $name_snake_case$Subscription;

//   TimelineBloc({@required this.$name_snake_case$Bloc}) {
//     $name_snake_case$Subscription = $name_snake_case$Bloc.state.listen((state) {
//       if (state is StartupEvent) {
//         // dispatch(UpdateTodos((todosBloc.currentState as TodosLoaded).todos));
//       }
//     });
//   }

//   @override
//   TimelineState get initialState {
//     // return $name_snake_case$Bloc.currentState is TodosLoaded
//     //     ? TimelineLoaded(
//     //         (todosBloc.currentState as TodosLoaded).todos,
//     //         VisibilityFilter.all,
//     //       )
//     //     : TimelineLoading();
//     return TimelineLoading();
//   }

//   @override
//   Stream<TimelineState> mapEventToState(TimelineEvent event) async* {
//     if (event is LoadTimeline) {
//       yield* _mapLoadTimelineToState(event);
//     // } else if (event is UpdateTodos) {
//     //   yield* _mapTodosUpdatedToState(event);
//     }
//   }

//   Stream<TimelineState> _mapLoadTimelineToState(LoadTimeline event) async* {
//     print("in load timeline to state");
//     yield TimelineLoaded([]);
//   }

//   // Stream<TimelineState> _mapUpdateFilterToState(
//   //   UpdateFilter event,
//   // ) async* {
//   //   if (todosBloc.currentState is TodosLoaded) {
//   //     yield TimelineLoaded(
//   //       _mapTodosToTimeline(
//   //         (todosBloc.currentState as TodosLoaded).todos,
//   //         event.filter,
//   //       ),
//   //       event.filter,
//   //     );
//   //   }
//   // }

//   // Stream<TimelineState> _mapTodosUpdatedToState(
//   //   UpdateTodos event,
//   // ) async* {
//   //   final visibilityFilter = currentState is TimelineLoaded
//   //       ? (currentState as TimelineLoaded).activeFilter
//   //       : VisibilityFilter.all;
//   //   yield TimelineLoaded(
//   //     _mapTodosToTimeline(
//   //       (todosBloc.currentState as TodosLoaded).todos,
//   //       visibilityFilter,
//   //     ),
//   //     visibilityFilter,
//   //   );
//   // }

//   // List<Todo> _mapTodosToTimeline(
//   //     List<Todo> todos, VisibilityFilter filter) {
//   //   return todos.where((todo) {
//   //     if (filter == VisibilityFilter.all) {
//   //       return true;
//   //     } else if (filter == VisibilityFilter.active) {
//   //       return !todo.complete;
//   //     } else if (filter == VisibilityFilter.completed) {
//   //       return todo.complete;
//   //     }
//   //   }).toList();
//   // }

//   @override
//   void dispose() {
//     $name_snake_case$Subscription.cancel();
//     super.dispose();
//   }
// }
