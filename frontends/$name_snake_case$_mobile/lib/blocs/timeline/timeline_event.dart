import 'package:meta/meta.dart';
import 'package:equatable/equatable.dart';

@immutable
abstract class TimelineEvent extends Equatable {
  TimelineEvent([List props = const []]) : super(props);
}

class LoadTimeline extends TimelineEvent {
  LoadTimeline();
  @override
  String toString() => "LoadTimeline";
}
