import 'package:blog_back/app/global/global_static.dart';
import 'package:blog_back/app/component/request_mixin.dart';
import 'package:blog_back/app/global/global_storage.dart';
import 'package:blog_back/app/pages/home_page/home_page.dart';
import 'package:blog_back/app/pages/login_page/component/login_recaptcha.dart';
import 'package:flutter/material.dart';

class LoginProvider with ChangeNotifier, RequestMixin {
  final GlobalKey<FormState> formKey = GlobalKey<FormState>();

  bool isRequesting = false;

  String username = "", password = "", authCode = "";

  String? onUsernameInputValidator(String? value) {
    if (value == null || value.isEmpty) {
      return "请输入用户名";
    }
    return null;
  }

  String? onPasswordInputValidator(String? value) {
    if (value == null || value.isEmpty) {
      return "请输入密码";
    }
    return null;
  }

  void onUsernameChanged(String value) => username = value;

  void onPasswordChanged(String value) => password = value;

  void onAuthCodeChanged(String value) => authCode = value;

  void onTapLogin() async {
    showDialog(
        context: GlobalStatic.navigatorKey.currentContext!,
        builder: (_) => const LoginRecaptcha());

    return;
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
    var route = PageRouteBuilder(
      transitionDuration: const Duration(milliseconds: 400),
      pageBuilder: (_, animation, secondary) {
        return SlideTransition(
            position: Tween(begin: const Offset(1, 0), end: const Offset(0, 0))
                .animate(animation),
            child: const HomePage());
      },
    );
    Navigator.of(GlobalStatic.navigatorKey.currentContext!)
        .pushReplacement(route);
  }
}
