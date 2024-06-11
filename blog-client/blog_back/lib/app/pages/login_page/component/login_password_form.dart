import 'package:flutter/material.dart';

class LoginPasswordForm extends StatefulWidget {
  const LoginPasswordForm({super.key, required this.validator, this.onChanged});

  final FormFieldValidator<String?> validator;

  final Function(String)? onChanged;

  @override
  State<LoginPasswordForm> createState() => _LoginPasswordFormState();
}

class _LoginPasswordFormState extends State<LoginPasswordForm> {
  bool obscureText = true;

  void onTapObscure() {
    obscureText = !obscureText;
    if (mounted) setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    return TextFormField(
      style: const TextStyle(
        color: Color(0xFF0D253C),
        fontSize: 16,
        fontWeight: FontWeight.w500,
      ),
      obscureText: obscureText,
      validator: widget.validator,
      onChanged: widget.onChanged,
      maxLength: 20,
      decoration: InputDecoration(
        labelText: "Password",
        counterText: "",
        suffixIcon: TextButton(
          onPressed: onTapObscure,
          child: Text(
            obscureText ? "show" : "hide",
            style: const TextStyle(
                color: Color(0xFF376AED),
                fontWeight: FontWeight.w500,
                fontSize: 14),
          ),
        ),
      ),
    );
  }
}
