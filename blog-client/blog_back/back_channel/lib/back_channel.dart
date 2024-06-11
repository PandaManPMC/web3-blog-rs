
import 'back_channel_platform_interface.dart';

class BackChannel {
  Future<String?> getDataDirectory() {
    return BackChannelPlatform.instance.getDataDirectory();
  }
}
