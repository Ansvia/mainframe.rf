import 'dart:async';
import 'dart:io' as io;

import 'package:path/path.dart';
import 'package:sqflite/sqflite.dart';
import 'package:path_provider/path_provider.dart';
import 'package:$name_snake_case$_mobile/models/models.dart';

class DatabaseHelper {
  static final DatabaseHelper _instance = new DatabaseHelper.internal();
  factory DatabaseHelper() => _instance;

  static Database _db;

  Future<Database> get db async {
    if(_db != null)
      return _db;
    _db = await initDb();
    return _db;
  }

  DatabaseHelper.internal();

  initDb() async {
    io.Directory documentsDirectory = await getApplicationDocumentsDirectory();
    String path = join(documentsDirectory.path, "main.db");
    var theDb = await openDatabase(path, version: 1, onCreate: _onCreate);
    return theDb;
  }
  
  void _onCreate(Database db, int version) async {
    // When creating the db, create the table
    await db.execute(
    "CREATE TABLE IF NOT EXISTS Account(id INTEGER PRIMARY KEY,full_name TEXT, email TEXT)");
    await db.execute(
    "CREATE TABLE IF NOT EXISTS AccessToken(token TEXT)");
    print("Tables created");
  }

  Future<bool> isLoggedIn() async {
    var dbClient = await db;
    var res = await dbClient.query("AccessToken");
    return res.length > 0;
  }

  Future<int> saveUser(Account account) async {
    var dbClient = await db;
    int res = await dbClient.insert("Account", account.toMap());
    return res;
  }

  Future<int> deleteUser() async {
    var dbClient = await db;
    int res = await dbClient.delete("Account");
    return res;
  }

  Future<Account> getUserInfo() async {
    var dbClient = await db;
    List<Map> list = await dbClient.rawQuery('SELECT * FROM Account ORDER BY id DESC LIMIT 1');
    if (list.length > 0){
      return Account.fromMap(list.first);
    }else{
      return null;
    }
  }
}
