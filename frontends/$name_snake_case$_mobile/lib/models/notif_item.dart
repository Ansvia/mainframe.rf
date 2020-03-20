import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';

@immutable
class NotifItem extends Equatable {
  final int id;
  final String kind;
  final String text;
  final int initiatorId;
  final int projectId;
  final List<String> keywords;
  final String ts;

  const NotifItem(this.id, this.kind, this.text, this.initiatorId, this.projectId, this.keywords, this.ts);

  @override
  List<Object> get props => [
    this.id,
    this.kind,
    this.text,
    this.initiatorId,
    this.projectId,
    this.keywords,
    this.ts
  ];
  
  @override
  String toString() {
    return ''' NotifItem{
      id: $id,
      kind: $kind,
      text: $text,
      initiatorId: $initiatorId,
      projectId: $projectId,
      keywords: $keywords,
      ts: $ts
    }
    ''';
  }
}
