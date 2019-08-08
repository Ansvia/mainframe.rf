import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:$name_snake_case$_mobile/core/core.dart';
import 'package:$name_snake_case$_mobile/models/models.dart';

typedef OnSaveCallback = Function(String text);

class AddCommentScreen extends StatefulWidget {
  // final bool isEditing;
  final OnSaveCallback onSave;
  final FeedItem feed;

  AddCommentScreen({
    Key key,
    @required this.onSave,
    // @required this.isEditing,
    this.feed,
  }) : super(key: key ?? $name_camel_case$Keys.addCommentScreen);

  @override
  _AddCommentScreenState createState() => _AddCommentScreenState();
}

class _AddCommentScreenState extends State<AddCommentScreen> {
  static final GlobalKey<FormState> _formKey = GlobalKey<FormState>();

  String _text;
  // String _note;

  // bool get isEditing => widget.isEditing;

  @override
  Widget build(BuildContext context) {
    final textTheme = Theme.of(context).textTheme;

    return Scaffold(
      appBar: AppBar(
        title: Text(
          "Add comment"
        ),
      ),
      body: Padding(
        padding: EdgeInsets.all(16.0),
        child: Form(
          key: _formKey,
          child: ListView(
            children: [
              // TextFormField(
              //   initialValue: '',
              //   key: $name_camel_case$Keys.commentField,
              //   autofocus: true,
              //   style: textTheme.headline,
              //   decoration: InputDecoration(
              //     hintText: "Add comment here",
              //   ),
              //   validator: (val) {
              //     return val.trim().isEmpty
              //         ? "No comment text"
              //         : null;
              //   },
              //   onSaved: (value) => _text = value,
              // ),
              TextFormField(
                initialValue: '',
                key: $name_camel_case$Keys.commentField,
                maxLines: 10,
                style: textTheme.subhead,
                decoration: InputDecoration(
                  hintText: "Add comment here",
                ),
                onSaved: (value) => _text = value,
              )
            ],
          ),
        ),
      ),
      floatingActionButton: FloatingActionButton(
        key:
            $name_camel_case$Keys.addFeedCommentButton,
        tooltip: "Add",
        child: Icon(Icons.add),
        onPressed: () {
          if (_formKey.currentState.validate()) {
            _formKey.currentState.save();
            widget.onSave(_text);
            Navigator.pop(context);
          }
        },
      ),
    );
  }
}
