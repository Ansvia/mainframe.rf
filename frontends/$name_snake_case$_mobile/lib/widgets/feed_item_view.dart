


import 'package:flutter/widgets.dart';
import 'package:$name_snake_case$_mobile/models/feed_item.dart';

class FeedItemView extends StatelessWidget {
  final FeedItem item;
  final GestureTapCallback onTap;

  FeedItemView({Key key, @required this.item, @required this.onTap}): super(key: key);

  @override
  Widget build(BuildContext context) {
    return new Container();
  }
}
