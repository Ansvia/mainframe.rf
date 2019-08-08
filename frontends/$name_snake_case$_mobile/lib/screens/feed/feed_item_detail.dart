import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:$name_snake_case$_mobile/core/core.dart';
import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_bloc.dart';
import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_event.dart';
import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_state.dart';
import 'package:$name_snake_case$_mobile/screens/feed/add_comment_screen.dart';

class FeedItemDetailScreen extends StatelessWidget {
  final String id;

  FeedItemDetailScreen({Key key, @required this.id})
      : super(key: key ?? $name_camel_case$Keys.feedItemDetailScreen);

  @override
  Widget build(BuildContext context) {
    final $name_snake_case$Bloc = BlocProvider.of<$name_camel_case$Bloc>(context);
    return BlocBuilder<$name_camel_case$Bloc, $name_camel_case$State>(
      builder: (context, state) {
        final feed = (state as TimelineLoaded)
            .feeds
            .firstWhere((feed) => feed.id == id, orElse: () => null);
        return Scaffold(
          appBar: AppBar(
            title: Text("Feed Detail"),
            actions: [
              IconButton(
                tooltip: "Delete this feed",
                key: $name_camel_case$Keys.deleteFeedButton,
                icon: Icon(Icons.delete),
                onPressed: () {
                  $name_snake_case$Bloc.dispatch(DeleteFeedItem(feed));
                  Navigator.pop(context, feed);
                },
              )
            ],
          ),
          body: feed == null
              ? Container(key: $name_camel_case$Keys.emptyFeedItemDetailContainer)
              : Padding(
                  padding: EdgeInsets.all(16.0),
                  child: ListView(
                    children: [
                      Row(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          // Padding(
                          //   padding: EdgeInsets.only(right: 8.0),
                          //   child: Checkbox(
                          //       key: FlutterTodosKeys.FeedItemDetailScreenCheckBox,
                          //       value: todo.complete,
                          //       onChanged: (_) {
                          //         $name_snake_case$Bloc.dispatch(
                          //           UpdateTodo(
                          //             todo.copyWith(complete: !todo.complete),
                          //           ),
                          //         );
                          //       }),
                          // ),
                          Expanded(
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Hero(
                                  tag: '${feed.id}__heroTag',
                                  child: Container(
                                    width: MediaQuery.of(context).size.width,
                                    padding: EdgeInsets.only(
                                      top: 8.0,
                                      bottom: 16.0,
                                    ),
                                    child: Text(
                                      feed.text,
                                      key: $name_camel_case$Keys.feedItemDetailFeedItemTask,
                                      style:
                                          Theme.of(context).textTheme.headline,
                                    ),
                                  ),
                                ),
                                Text(
                                  feed.text,
                                  key: $name_camel_case$Keys.feedItemDetailFeedItemNote,
                                  style: Theme.of(context).textTheme.subhead,
                                ),
                              ],
                            ),
                          ),
                        ],
                      ),
                    ],
                  ),
                ),
          floatingActionButton: FloatingActionButton(
            key: $name_camel_case$Keys.feedAddComment,
            tooltip: "Add comment",
            child: Icon(Icons.edit),
            onPressed: feed == null
                ? null
                : () {
                    Navigator.of(context).push(
                      MaterialPageRoute(
                        builder: (context) {
                          return AddCommentScreen(
                            key: $name_camel_case$Keys.addCommentScreen,
                            onSave: (text) {
                              $name_snake_case$Bloc.dispatch(
                                AddComment(
                                  text
                                ),
                              );
                            }
                          );
                        },
                      ),
                    );
                  },
          ),
        );
      },
    );
  }
}
