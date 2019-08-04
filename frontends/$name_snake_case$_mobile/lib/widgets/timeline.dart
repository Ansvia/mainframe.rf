import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:$name_snake_case$_mobile/core/$name_snake_case$_app_core.dart';
import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_bloc.dart';
import 'package:$name_snake_case$_mobile/blocs/$name_snake_case$/$name_snake_case$_state.dart';
import 'package:$name_snake_case$_mobile/screens/feed/feed_item_detail.dart';
import 'package:$name_snake_case$_mobile/widgets/widgets.dart';
import 'package:$name_snake_case$_mobile/widgets/loading_indicator.dart';

class Timeline extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    // final $name_snake_case$Bloc = BlocProvider.of<$name_camel_case$Bloc>(context);

    return BlocBuilder<$name_camel_case$Bloc, $name_camel_case$State>(
      builder: (context, state) {
        if (state is TimelineLoading) {
          return LoadingIndicator(key: $name_camel_case$Keys.timelineLoading);
        } else if (state is TimelineLoaded) {
          // return Text("satu");
          final feeds = state.feeds;
          return ListView.builder(
            key: $name_camel_case$Keys.timelineList,
            itemCount: feeds.length,
            itemBuilder: (BuildContext context, int index) {
              final item = feeds[index];
              return new FeedItemView(
                  item: item,
                  onTap: () async {
                    Navigator.of(context).push(MaterialPageRoute(builder: (_) {
                      return FeedItemDetailScreen(id: item.id);
                    }));
                  });
            },
          );
        } else {
          return LoadingIndicator(key: $name_camel_case$Keys.timelineLoading);
        }
      },
    );
  }
}
