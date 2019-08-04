
import 'package:meta/meta.dart';
import 'package:equatable/equatable.dart';
import 'package:$name_snake_case$_mobile/models/models.dart';

@immutable
abstract class TimelineState extends Equatable {
  TimelineState([List props = const []]): super(props);
}

// class TimelineLoading extends TimelineState {
//   TimelineLoading();

//   @override
//   String toString() => "TimelineLoading";
// }


// class TimelineLoaded extends TimelineState {
//   final List<FeedItem> feeds;

//   TimelineLoaded(this.feeds): super([feeds]);

//   @override
//   String toString() => "TimelineLoaded";
// }
