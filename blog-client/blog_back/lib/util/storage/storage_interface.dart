class StorageInterface {
  final String storageKey;
  StorageInterface(this.storageKey);

  Map<String, dynamic> storageData = {};

  Future<void> init() async {}

  T getValue<T>(String key) => storageData[key] as T;

  void setValue(String key, dynamic value) => storageData[key] = value;

  void clear() => storageData.clear();
}
