import 'package:plugin_platform_interface/plugin_platform_interface.dart';

import 'back_channel_method_channel.dart';

abstract class BackChannelPlatform extends PlatformInterface {
  /// Constructs a BackChannelPlatform.
  BackChannelPlatform() : super(token: _token);

  static final Object _token = Object();

  static BackChannelPlatform _instance = MethodChannelBackChannel();

  /// The default instance of [BackChannelPlatform] to use.
  ///
  /// Defaults to [MethodChannelBackChannel].
  static BackChannelPlatform get instance => _instance;

  /// Platform-specific implementations should set this with their own
  /// platform-specific class that extends [BackChannelPlatform] when
  /// they register themselves.
  static set instance(BackChannelPlatform instance) {
    PlatformInterface.verifyToken(instance, _token);
    _instance = instance;
  }

  Future<String?> getPlatformVersion() {
    throw UnimplementedError('platformVersion() has not been implemented.');
  }
}
