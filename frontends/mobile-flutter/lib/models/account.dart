class Account {
  String title;
  String subtitle;
  int id;

  Account({this.id, this.title, this.subtitle});

  factory Account.fromJson(Map<String, dynamic> json) {
    return Account(
        id: json['id'] as int,
        title: json['full_name'] as String,
        subtitle: json['email'] as String
    );
  }
}