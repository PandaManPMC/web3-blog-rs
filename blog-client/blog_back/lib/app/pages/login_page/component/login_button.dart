import 'package:blog_back/app/pages/login_page/login_provider.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class LoginButton extends StatelessWidget {
  const LoginButton({super.key});

  @override
  Widget build(BuildContext context) {
    return Consumer<LoginProvider>(builder: (_, provider, __) {
      Widget child = provider.isRequesting
          ? const CupertinoActivityIndicator(color: Color(0xFFFFFFFF))
          : const Text(
              "LOGIN",
              style: TextStyle(
                  color: Color(0xFFFFFFFF),
                  fontWeight: FontWeight.w500,
                  fontSize: 16),
            );

      return ElevatedButton(
        onPressed: provider.onTapLogin,
        style: ElevatedButton.styleFrom(
          backgroundColor: const Color(0xFF376AED),
          padding: EdgeInsets.zero,
          side: BorderSide.none,
          fixedSize: const Size(280, 50),
          shape:
              RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
        ),
        child: Center(child: child),
      );
    });
  }
}
