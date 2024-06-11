import 'package:blog_back/app/pages/login_page/login_provider.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:provider/provider.dart';
import 'login_password_form.dart';

class LoginFormComponent extends StatelessWidget {
  const LoginFormComponent({super.key});

  @override
  Widget build(BuildContext context) {
    LoginProvider provider = context.read<LoginProvider>();

    return Form(
      key: provider.formKey,
      child: Column(children: [
        TextFormField(
          style: const TextStyle(
            color: Color(0xFF0D253C),
            fontSize: 16,
            fontWeight: FontWeight.w500,
          ),
          validator: provider.onUsernameInputValidator,
          onChanged: provider.onUsernameChanged,
          maxLength: 20,
          decoration: const InputDecoration(
            labelText: "Username",
            counterText: "",
          ),
        ),
        const SizedBox(height: 10),
        LoginPasswordForm(
          validator: provider.onPasswordInputValidator,
          onChanged: provider.onPasswordChanged,
        ),
        const SizedBox(height: 10),
        TextFormField(
          style: const TextStyle(
            color: Color(0xFF0D253C),
            fontSize: 16,
            fontWeight: FontWeight.w500,
          ),
          onChanged: provider.onAuthCodeChanged,
          maxLength: 6,
          inputFormatters: [FilteringTextInputFormatter.digitsOnly],
          decoration: const InputDecoration(
            labelText: "GoogleAuth Code",
            counterText: "",
          ),
        ),
      ]),
    );
  }
}
