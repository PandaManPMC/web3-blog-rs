import 'dart:ui';

import 'package:blog_back/component/app_theme.dart';
import 'package:blog_back/app/global/global_static.dart';
import 'package:blog_back/app/global/global_storage.dart';
import 'package:blog_back/app/pages/splash_page/splash_page.dart';
import 'package:flutter/material.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await GlobalStorage().initialize().then((_) => runApp(const App()));
}

class App extends StatelessWidget {
  const App({super.key});

  void onTapScaffold(BuildContext context) {
    FocusScopeNode currentFocus = FocusScope.of(context);
    if (!currentFocus.hasPrimaryFocus && currentFocus.focusedChild != null) {
      FocusManager.instance.primaryFocus?.unfocus();
    }
  }

  Widget materialAppBuilder(BuildContext context, Widget? child) {
    child = GestureDetector(onTap: () => onTapScaffold(context), child: child);
    return child;
  }

  ScrollBehavior get scrollBehavior => const ScrollBehavior().copyWith(
        overscroll: true,
        physics: const ClampingScrollPhysics(),
        dragDevices: PointerDeviceKind.values.toSet(),
      );

  @override
  Widget build(BuildContext context) {
    return MediaQuery(
      data: MediaQuery.of(context).copyWith(textScaleFactor: 1.0),
      child: MaterialApp(
        navigatorKey: GlobalStatic.navigatorKey,
        scrollBehavior: scrollBehavior,
        builder: materialAppBuilder,
        debugShowCheckedModeBanner: false,
        theme: appTheme,
        home: const SplashPage(),
      ),
    );
  }
}
