import 'package:blog_back/app/component/request_mixin.dart';
import 'package:blog_back/app/global/global_static.dart';
import 'package:blog_back/app/global/global_storage.dart';
import 'package:flutter/material.dart';

class TotpProvider with ChangeNotifier, RequestMixin {
  String secretKey = "", secretLink = "";

  String authCode = "";

  bool isRequesting = false;

  final GlobalKey<FormState> formKey = GlobalKey<FormState>();

  void requestAuthSecret() async {
    await requestGet(GlobalStatic.httpTotpLink, success: onSecretKeySuccess);
  }

  void onSecretKeySuccess(data) {
    secretKey = data["secret"] ?? "";
    secretLink = data["qrCodeUrl"] ?? "";
    notifyListeners();
  }

  @override
  void dispose() {
    super.dispose();
    token.cancel();
  }

  void onAuthCodeChanged(String value) => authCode = value;

  String? onAuthCodeInputValidator(String? value) {
    if (value == null || value.isEmpty) {
      return "请输入验证码";
    }
    if (value.length != 6) {
      return "验证码的长度是6";
    }
    return null;
  }

  void onTapBind() async {
    if (isRequesting) return;
    if (secretKey.isEmpty || secretLink.isEmpty) return;
    if (!formKey.currentState!.validate()) return;
    isRequesting = true;
    notifyListeners();
    await requestPost(
      GlobalStatic.httpTotpBind,
      params: {"googleAuthCode": authCode},
      success: onTotpBindSuccess,
    );
    isRequesting = false;
    notifyListeners();
  }

  void onTotpBindSuccess(data) {
    showToast("绑定成功");
    GlobalStorage().onTotpAuth();
    Navigator.of(GlobalStatic.navigatorKey.currentContext!).pop();
  }
}
