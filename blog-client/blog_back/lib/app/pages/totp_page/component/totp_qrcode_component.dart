import 'package:blog_back/app/pages/totp_page/totp_provider.dart';
import 'package:flutter/cupertino.dart';
import 'package:pretty_qr_code/pretty_qr_code.dart';
import 'package:provider/provider.dart';

class TotpQrcodeComponent extends StatelessWidget {
  const TotpQrcodeComponent({super.key});

  @override
  Widget build(BuildContext context) {
    String secretLink = context.watch<TotpProvider>().secretLink;
    return Container(
      width: 160,
      height: 160,
      alignment: Alignment.center,
      padding: const EdgeInsets.all(10),
      decoration: BoxDecoration(
        color: const Color(0xFFFFFFFF),
        border: Border.all(color: const Color(0xFF7B8BB2), width: 1),
        borderRadius: BorderRadius.circular(8),
      ),
      child: secretLink.isEmpty
          ? const CupertinoActivityIndicator()
          : PrettyQrView.data(
              data: secretLink,
              errorCorrectLevel: QrErrorCorrectLevel.H,
            ),
    );
  }
}
