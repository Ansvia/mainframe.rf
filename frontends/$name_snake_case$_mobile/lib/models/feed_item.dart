import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';

@immutable
class FeedItem extends Equatable {
  final String id;
  final String creatorName;
  final String text;
  final String time;

  FeedItem(this.id, this.creatorName, this.text, this.time): super([id, creatorName, text, time]);
  
}