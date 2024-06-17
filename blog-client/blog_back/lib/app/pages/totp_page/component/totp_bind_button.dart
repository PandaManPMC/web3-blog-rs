import 'package:blog_back/app/pages/totp_page/totp_provider.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class TotpBindButton extends StatelessWidget {
  const TotpBindButton({super.key});

  @override
  Widget build(BuildContext context) {
    return Consumer<TotpProvider>(builder: (_, provider, __) {
      Widget child = provider.isRequesting
          ? const CupertinoActivityIndicator(color: Color(0xFFFFFFFF))
          : const Text(
              "BIND TOTP",
              style: TextStyle(
                  color: Color(0xFFFFFFFF),
                  fontWeight: FontWeight.w500,
                  fontSize: 16),
            );

      return ElevatedButton(
        onPressed: provider.onTapBind,
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
