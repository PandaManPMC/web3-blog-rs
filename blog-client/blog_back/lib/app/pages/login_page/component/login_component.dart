import 'package:blog_back/app/pages/login_page/component/login_button.dart';
import 'package:flutter/material.dart';

import 'login_form_component.dart';

class LoginComponent extends StatelessWidget {
  const LoginComponent({super.key});

  @override
  Widget build(BuildContext context) {
    const borderRadius = BorderRadius.only(
      topLeft: Radius.circular(28),
      topRight: Radius.circular(28),
    );

    return Container(
      height: 460,
      width: double.infinity,
      padding: const EdgeInsets.fromLTRB(20, 30, 20, 20),
      decoration: const BoxDecoration(
        color: Color(0xFFFFFFFF),
        borderRadius: borderRadius,
      ),
      child: const Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            "Welcome back",
            style: TextStyle(
              fontSize: 24,
              color: Color(0xFF0D253C),
              fontWeight: FontWeight.w600,
            ),
          ),
          Text(
            "Sign in with your account",
            style: TextStyle(
              color: Color(0xFF2D4379),
              fontSize: 14,
            ),
          ),
          SizedBox(height: 30),
          LoginFormComponent(),
          SizedBox(height: 30),
          Center(child: LoginButton())
        ],
      ),
    );
  }
}
