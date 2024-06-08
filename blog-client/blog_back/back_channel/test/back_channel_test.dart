import 'package:flutter_test/flutter_test.dart';
import 'package:back_channel/back_channel.dart';
import 'package:back_channel/back_channel_platform_interface.dart';
import 'package:back_channel/back_channel_method_channel.dart';
import 'package:plugin_platform_interface/plugin_platform_interface.dart';

class MockBackChannelPlatform
    with MockPlatformInterfaceMixin
    implements BackChannelPlatform {

  @override
  Future<String?> getPlatformVersion() => Future.value('42');
}

void main() {
  final BackChannelPlatform initialPlatform = BackChannelPlatform.instance;

  test('$MethodChannelBackChannel is the default instance', () {
    expect(initialPlatform, isInstanceOf<MethodChannelBackChannel>());
  });

  test('getPlatformVersion', () async {
    BackChannel backChannelPlugin = BackChannel();
    MockBackChannelPlatform fakePlatform = MockBackChannelPlatform();
    BackChannelPlatform.instance = fakePlatform;

    expect(await backChannelPlugin.getPlatformVersion(), '42');
  });
}
