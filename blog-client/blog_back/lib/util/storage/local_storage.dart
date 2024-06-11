import 'dart:async';

import 'storage_html.dart' if (dart.library.io) 'storage_io.dart';

class LocalStorage {
  bool _initialized = false;
  bool get initialized => _initialized;

  late Storage _storage;

  Future<bool> initialize({String fileKey = "Storage"}) async {
    if (_initialized) return true;
    _storage = Storage(fileKey);
    await _storage.init();
    _initialized = true;
    return true;
  }

  bool save(String key, dynamic value) {
    if (!_initialized) return false;
    scheduleMicrotask(() => _storage.setValue(key, value));
    return true;
  }

  T? read<T>(String key) => !_initialized ? null : _storage.getValue(key);

  void clear() => _storage.clear();
}
