import 'package:blog_back/util/storage/local_storage.dart';
import 'package:flutter/foundation.dart';

enum GlobalStorageKey { user }

extension GlobalStorageKeyExtension on GlobalStorageKey {
  String get name => ["user"][index];
}

class GlobalStorage {
  factory GlobalStorage() => _instance;
  static final _instance = GlobalStorage._();

  GlobalStorage._();

  final LocalStorage storage = LocalStorage();

  String _accessToken = "";
  String get accessToken => _accessToken;

  String _username = "";
  String get username => _username;

  bool _hasTotpAuth = false;
  bool get hasTotpAuth => _hasTotpAuth;

  Future<void> initialize() async {
    await storage.initialize(fileKey: "blog-storage");
    readUserInfo();
    debugPrint(toString());
  }

  void readUserInfo() {
    Map user = storage.read(GlobalStorageKey.user.name) ?? {};
    _accessToken = user["userToken"] ?? "";
    _username = user["penName"] ?? "";
    _hasTotpAuth = user["googleAuth"] ?? false;
  }

  void saveUserInfo(user) {
    try {
      _accessToken = user["userToken"] ?? "";
      _username = user["penName"] ?? "";
      _hasTotpAuth = user["googleAuth"] ?? false;
      storage.save(GlobalStorageKey.user.name, user);
    } catch (_) {}
  }

  void onTotpAuth() {
    _hasTotpAuth = true;
    Map user = {
      "userToken": _accessToken,
      "penName": _username,
      "googleAuth": true,
    };
    storage.save(GlobalStorageKey.user.name, user);
  }

  @override
  String toString() => "\x1B[32m AccessToken: $_accessToken\x1B[0m";
}
