import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:back_channel/back_channel.dart';
import 'package:blog_back/util/storage/storage_interface.dart';

class Storage extends StorageInterface {
  Storage(super.storageKey);

  RandomAccessFile? _accessFile;

  @override
  Future<void> init() async {
    await _initAccessFile();
    int lengthSync = _accessFile?.lengthSync() ?? 0;
    if (lengthSync == 0) {
      await _writeToStorage();
    } else {
      await _readFromStorage();
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
    _writeToStorage();
  }

  Future<void> _initAccessFile() async {
    String? path = await BackChannel().getDataDirectory();
    if (path == null || path.isEmpty) return;
    path = "$path/$storageKey.gs";
    File dataFile = File(path);
    if (!dataFile.existsSync()) {
      dataFile.createSync(recursive: true);
    }
    _accessFile = await dataFile.open(mode: FileMode.append);
  }

  Future<void> _writeToStorage() async {
    final buffer = utf8.encode(json.encode(storageData));
    final length = buffer.length;

    await _accessFile?.lock();
    await _accessFile?.setPosition(0);
    await _accessFile?.writeFrom(buffer);
    await _accessFile?.truncate(length);
    await _accessFile?.unlock();
  }

  Future<void> _readFromStorage() async {
    await _accessFile?.setPosition(0);
    final buffer = Uint8List(await _accessFile?.length() ?? 0);
    await _accessFile?.readInto(buffer);
    storageData = json.decode(utf8.decode(buffer)) as Map<String, dynamic>;
  }
}
