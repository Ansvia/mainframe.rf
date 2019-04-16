class $param.service_name_camel_case$ {
  String title;
  String subtitle;
  int id;

  $param.service_name_camel_case$({this.id, this.title, this.subtitle});

  factory $param.service_name_camel_case$.fromJson(Map<String, dynamic> json) {
    return $param.service_name_camel_case$(
        id: json['id'] as int,
        title: json['full_name'] as String,
        subtitle: json['email'] as String
    );
  }
}