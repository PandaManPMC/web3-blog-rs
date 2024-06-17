import 'package:blog_back/app/pages/totp_page/totp_provider.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class TotpSecretComponent extends StatelessWidget {
  const TotpSecretComponent({super.key});

  @override
  Widget build(BuildContext context) {
    String secretKey = context.watch<TotpProvider>().secretKey;
    return Container(
      height: 50,
      padding: const EdgeInsets.symmetric(horizontal: 10),
      decoration: BoxDecoration(
        color: const Color(0xFF376AED).withOpacity(0.2),
        borderRadius: BorderRadius.circular(8),
      ),
      child: Row(children: [
        Expanded(
          child: secretKey.isEmpty
              ? const Align(
                  alignment: Alignment.centerLeft,
                  child: CupertinoActivityIndicator(),
                )
              : Text(
                  secretKey,
                  style:
                      const TextStyle(color: Color(0xFF0D253C), fontSize: 14),
                ),
        ),
        const Icon(Icons.copy, color: Color(0xFF000000), size: 20),
      ]),
    );
  }
}
