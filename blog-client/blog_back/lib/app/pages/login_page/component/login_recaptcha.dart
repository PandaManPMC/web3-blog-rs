import 'package:flutter/material.dart';
import 'package:webview_flutter/webview_flutter.dart';

class LoginRecaptcha extends StatefulWidget {
  const LoginRecaptcha({super.key});

  @override
  State<LoginRecaptcha> createState() => _LoginRecaptchaState();
}

class _LoginRecaptchaState extends State<LoginRecaptcha> {
  late WebViewController controller;

  @override
  void initState() {
    super.initState();
    var params = const PlatformWebViewControllerCreationParams();
    controller = WebViewController.fromPlatformCreationParams(params);
    controller.clearCache();
    controller.setJavaScriptMode(JavaScriptMode.unrestricted);
    controller.addJavaScriptChannel("JsBridge", onMessageReceived: (msg) {
      print(msg.message);
    });
    controller.loadFlutterAsset("recaptcha/index.html");
  }

  @override
  Widget build(BuildContext context) {
    return WebViewWidget(controller: controller);
  }
}
