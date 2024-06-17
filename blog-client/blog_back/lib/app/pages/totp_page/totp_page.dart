import 'package:blog_back/app/pages/totp_page/component/totp_bind_button.dart';
import 'package:blog_back/app/pages/totp_page/component/totp_qrcode_component.dart';
import 'package:blog_back/app/pages/totp_page/component/totp_secret_component.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:provider/provider.dart';
import 'totp_provider.dart';

class TotpPage extends StatefulWidget {
  const TotpPage({super.key});

  @override
  State<TotpPage> createState() => _TotpPageState();
}

class _TotpPageState extends State<TotpPage> {
  final totpProvider = TotpProvider();

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      totpProvider.requestAuthSecret();
    });
  }

  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider(
      create: (_) => totpProvider,
      child: Scaffold(
        appBar: AppBar(
          title: const Text("New Article"),
          titleTextStyle: const TextStyle(
            color: Color(0xFF0D253C),
            fontSize: 24,
            fontWeight: FontWeight.w600,
          ),
        ),
        body: SingleChildScrollView(
          padding: const EdgeInsets.fromLTRB(20, 30, 20, 0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              const Center(child: TotpQrcodeComponent()),
              const SizedBox(height: 10),
              const Text(
                "密钥",
                style: TextStyle(
                  color: Color(0xFF0D253C),
                  fontSize: 16,
                  fontWeight: FontWeight.w600,
                ),
              ),
              const SizedBox(height: 20),
              const TotpSecretComponent(),
              const SizedBox(height: 10),
              const Text(
                "1.下载并安装TOTP验证器APP，如Google Authenticator",
                style: TextStyle(color: Color(0xFF0D253C), fontSize: 12),
              ),
              const Text(
                "2.在验证器APP中添加你的账户名和上方密钥",
                style: TextStyle(color: Color(0xFF0D253C), fontSize: 12),
              ),
              const SizedBox(height: 20),
              const Text(
                "注意：请妥善保管此密钥，用于手机更换或遗失时恢复TOTP验证器。Blog-Club不提供密钥找回服务。",
                style: TextStyle(color: Color(0xFFFF3743), fontSize: 12),
              ),
              const SizedBox(height: 40),
              Form(
                key: totpProvider.formKey,
                child: TextFormField(
                  style: const TextStyle(
                    color: Color(0xFF0D253C),
                    fontSize: 16,
                    fontWeight: FontWeight.w500,
                  ),
                  maxLength: 6,
                  onChanged: totpProvider.onAuthCodeChanged,
                  validator: totpProvider.onAuthCodeInputValidator,
                  inputFormatters: [FilteringTextInputFormatter.digitsOnly],
                  decoration: const InputDecoration(
                    labelText: "TOTP Code",
                    border: OutlineInputBorder(
                      borderSide: BorderSide(color: Color(0xFF376AED)),
                    ),
                    enabledBorder: OutlineInputBorder(
                      borderSide: BorderSide(color: Color(0xFF376AED)),
                    ),
                    focusedBorder: OutlineInputBorder(
                      borderSide: BorderSide(color: Color(0xFF376AED)),
                    ),
                  ),
                ),
              ),
              const SizedBox(height: 40),
              const TotpBindButton(),
            ],
          ),
        ),
      ),
    );
  }
}
