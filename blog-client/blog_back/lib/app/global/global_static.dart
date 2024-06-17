import 'package:flutter/material.dart';

class GlobalStatic {
  static GlobalKey<NavigatorState> navigatorKey = GlobalKey<NavigatorState>();

  static const String httpBaseLink = "http://192.168.30.12:51080";

  //
  static const String httpLogin = "/admin/login";

  //
  static const String httpTotpLink = "/admin/getStartBindGoogleSecret";

  //
  static const String httpTotpBind = "/admin/bindGoogleSecret";
}
