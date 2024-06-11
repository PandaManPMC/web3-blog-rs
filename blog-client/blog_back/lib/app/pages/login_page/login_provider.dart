import 'package:blog_back/app/global/global_static.dart';
import 'package:blog_back/app/component/request_mixin.dart';
import 'package:blog_back/app/global/global_storage.dart';
import 'package:flutter/material.dart';

class LoginProvider with ChangeNotifier, RequestMixin {
  final GlobalKey<FormState> formKey = GlobalKey<FormState>();

  bool isRequesting = false;

  String username = "", password = "", authCode = "";

  String? onUsernameInputValidator(String? value) {
    if (value == null || value.isEmpty) {
      return "please enter your username";
    }
    return null;
  }

  String? onPasswordInputValidator(String? value) {
    if (value == null || value.isEmpty) {
      return "please enter your password";
    }
    return null;
  }

  void onUsernameChanged(String value) => username = value;

  void onPasswordChanged(String value) => password = value;

  void onAuthCodeChanged(String value) => authCode = value;

  void onTapLogin() async {
    if (isRequesting) return;
    if (!formKey.currentState!.validate()) return;

    isRequesting = true;
    notifyListeners();
    await requestPost(
      GlobalStatic.httpLogin,
      params: {
        "userName": username,
        "userPwd": password,
        "googleAuthCode": authCode
      },
      success: onLoginSuccess,
    );
    isRequesting = false;
    notifyListeners();
  }

  void onLoginSuccess(data) {
    GlobalStorage().saveUserInfo(data);
  }
}
