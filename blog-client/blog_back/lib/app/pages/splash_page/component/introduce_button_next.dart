import 'package:blog_back/app/global/global_storage.dart';
import 'package:blog_back/app/pages/home_page/home_page.dart';
import 'package:blog_back/app/pages/login_page/login_page.dart';
import 'package:flutter/material.dart';

class IntroduceButtonNext extends StatelessWidget {
  const IntroduceButtonNext({super.key});

  void onTapNext(BuildContext context) {
    if (GlobalStorage().accessToken.isEmpty) {
      var route = PageRouteBuilder(
        transitionDuration: const Duration(milliseconds: 600),
        pageBuilder: (_, animation, secondary) {
          return FadeTransition(opacity: animation, child: const LoginPage());
        },
      );
      Navigator.of(context).push(route);
    } else {
      var route = PageRouteBuilder(
        transitionDuration: const Duration(milliseconds: 400),
        pageBuilder: (_, animation, secondary) {
          return SlideTransition(
              position:
                  Tween(begin: const Offset(1, 0), end: const Offset(0, 0))
                      .animate(animation),
              child: const HomePage());
        },
      );
      Navigator.of(context).pushReplacement(route);
    }
  }

  @override
  Widget build(BuildContext context) {
    return ElevatedButton(
      onPressed: () => onTapNext(context),
      style: ElevatedButton.styleFrom(
        backgroundColor: const Color(0xFF376AED),
        padding: EdgeInsets.zero,
        side: BorderSide.none,
        fixedSize: const Size(80, 50),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
      ),
      child: Center(
        child: Image.asset(
          "images/arrow_right.png",
          width: 24,
          height: 24,
        ),
      ),
    );
  }
}
