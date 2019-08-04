import 'package:flutter/widgets.dart';

class $name_camel_case$Keys {
  // Home Screens
  static final homeScreen = const Key('__homeScreen__');
  static final addTodoFab = const Key('__addTodoFab__');
  static final snackbar = const Key('__snackbar__');
  static Key snackbarAction(String id) => Key('__snackbar_action_${id}__');

  // Timeline
  static final timelineLoading = const Key('__timelineLoading__');
  static final timelineItem = (String id) => Key('TimelineItem__${id}');
  static final timelineItemTask = (String id) => Key('TimelineItem__${id}__Task');
  static final timelineItemNote = (String id) => Key('TimelineItem__${id}__Note');
  static final timelineList = Key('__timelineList__');

  // Feed
  static final feeds = const Key('__feeds__');
  static final feedItemDetail = const Key('__feedItemDetail__');
  static final feedItemDetailScreen = const Key('__feedItemDetailScreen__');
  static final emptyFeedItemDetailContainer = const Key('__emptyFeedItemDetailContainer__');
  static final feedItemDetailFeedItemTask = const Key('__feedItemDetailFeedItemTask__');
  static final feedItemDetailFeedItemNote = const Key('__feedItemDetailTodoItemNote__');
  static final feedAddComment = const Key('__feedAddComment__');

  // Task
  static final addTask = const Key('__addTask__');

  // Screen
  static final addCommentScreen = const Key('__addCommentScreen__');

  // Tabs
  static final tabs = const Key('__tabs__');
  static final timelineTab = const Key('__timelineTab__');
  static final notifTab = const Key('__notifTab__');
  static final todoTab = const Key('__todoTab__');
  static final dashboardTab = const Key('__dashboardTab__');

  // Notif
  static final notifList = const Key('__notifList__');

  // Buttons
  static final deleteFeedButton = const Key('__deleteFeedButton__');
  static final addFeedCommentButton = const Key('__addFeedCommentButton__');
  static final updateStatusFab = const Key('__updateStatusFab__');

  // etc
  static final commentField = const Key('__commentField__');
  static final loading = const Key('__loading__');
  static final logo = "__logo__";
}

class $name_camel_case$Routes {
  static final login = "/login";
  
  static final taskMan = "/taskman";
  static final updateStatus = "/update-status";
  static final addComment = "/add-comment";
  
}
