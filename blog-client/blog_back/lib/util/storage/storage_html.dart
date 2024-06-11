import 'dart:html' as html;
import 'dart:convert';

import 'package:blog_back/util/storage/storage_interface.dart';

class Storage extends StorageInterface {
  Storage(super.storageKey);

  final html.Storage _storage = html.window.localStorage;

  @override
  Future<void> init() async {
    bool exitStorageFile = _storage.containsKey(storageKey);
    if (exitStorageFile) {
      _readFromStorage();
    } else {
      _writeToStorage();
    }
  }

  @override
  void setValue(String key, dynamic value) {
    super.setValue(key, value);
    _writeToStorage();
  }

  @override
  void clear() {
    super.clear();
    _storage.clear();
  }

  void _writeToStorage() {
    _storage.update(storageKey, (value) => json.encode(storageData),
        ifAbsent: () => json.encode(storageData));
  }

  void _readFromStorage() {
    MapEntry<String, String> data = _storage.entries.firstWhere(
      (e) => e.key == storageKey,
      orElse: () => MapEntry(storageKey, jsonEncode(storageData)),
    );

    storageData = jsonDecode(data.value);
  }
}
