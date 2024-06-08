
import 'back_channel_platform_interface.dart';

class BackChannel {
  Future<String?> getPlatformVersion() {
    return BackChannelPlatform.instance.getPlatformVersion();
  }
}
