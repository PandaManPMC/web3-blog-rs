import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import 'component/login_component.dart';
import 'login_provider.dart';

class LoginPage extends StatefulWidget {
  const LoginPage({super.key});

  @override
  State<LoginPage> createState() => _LoginPageState();
}

class _LoginPageState extends State<LoginPage>
    with SingleTickerProviderStateMixin {
  late AnimationController animCtr;

  @override
  void initState() {
    super.initState();
    animCtr = AnimationController(
        duration: const Duration(milliseconds: 800), vsync: this);
    playPageAnim();
  }

  @override
  void deactivate() {
    super.deactivate();
    animCtr.stop();
  }

  @override
  void dispose() {
    super.dispose();
    animCtr.dispose();
  }

  void playPageAnim() async {
    await Future.delayed(const Duration(milliseconds: 350));
    animCtr.forward();
  }

  @override
  Widget build(BuildContext context) {
    var top = MediaQuery.of(context).padding.top;
    const borderRadius = BorderRadius.only(
      topLeft: Radius.circular(28),
      topRight: Radius.circular(28),
    );

    return ChangeNotifierProvider(
      create: (_) => LoginProvider(),
      child: Scaffold(
        body: Column(
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            SizedBox(width: double.infinity, height: 120 + top),
            Image.asset("images/logo.png", height: 56),
          ],
        ),
        bottomSheet: Container(
          padding: const EdgeInsets.only(top: 50),
          decoration: const BoxDecoration(
              borderRadius: borderRadius, color: Color(0xFF376AED)),
          child: SlideTransition(
            position: Tween<Offset>(
                    begin: const Offset(0, 2), end: const Offset(0, 0))
                .animate(CurvedAnimation(
                    parent: animCtr, curve: Curves.fastOutSlowIn)),
            child: const LoginComponent(),
          ),
        ),
      ),
    );
  }
}
