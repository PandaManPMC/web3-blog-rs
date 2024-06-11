import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

import 'back_channel_platform_interface.dart';

/// An implementation of [BackChannelPlatform] that uses method channels.
class MethodChannelBackChannel extends BackChannelPlatform {
  /// The method channel used to interact with the native platform.
  @visibleForTesting
  final methodChannel = const MethodChannel('back_channel');

  @override
  Future<String?> getDataDirectory() async {
    final path = await methodChannel.invokeMethod<String>('getDataDirectory');
    return path;
  }
}
