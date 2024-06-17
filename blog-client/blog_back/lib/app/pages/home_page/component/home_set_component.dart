import 'package:blog_back/app/global/global_storage.dart';
import 'package:blog_back/app/pages/category_page/category_page.dart';
import 'package:blog_back/app/pages/home_page/component/set_menu_title.dart';
import 'package:blog_back/app/pages/home_page/component/user_profile_component.dart';
import 'package:blog_back/app/pages/password_page/password_page.dart';
import 'package:blog_back/app/pages/totp_page/totp_page.dart';
import 'package:flutter/material.dart';

class HomeSetComponent extends StatefulWidget {
  const HomeSetComponent({super.key});

  @override
  State<HomeSetComponent> createState() => _HomeSetComponentState();
}

class _HomeSetComponentState extends State<HomeSetComponent> {
  int menuIndex = 0;

  void onTapMenu(int index) {
    if (menuIndex != index) {
      menuIndex = index;
      if (mounted) setState(() {});
    }

    Widget page;
    if (index == 1) {
      if (GlobalStorage().hasTotpAuth) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text("已绑定")),
        );
        return;
      }
      page = const TotpPage();
    } else if (index == 2) {
      page = const PasswordPage();
    } else {
      page = const CategoryPage();
    }

    var route = PageRouteBuilder(
      transitionDuration: const Duration(milliseconds: 400),
      pageBuilder: (_, animation, secondary) {
        return SlideTransition(
            position: Tween(begin: const Offset(-1, 0), end: const Offset(0, 0))
                .animate(animation),
            child: page);
      },
    );
    Navigator.of(context).push(route);
  }

  @override
  Widget build(BuildContext context) {
    return SafeArea(
      child: SizedBox(
        width: 288,
        height: double.infinity,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const UserProfileComponent(),
            const Padding(
              padding: EdgeInsets.fromLTRB(24, 32, 0, 16),
              child: Text(
                "BROWSE",
                style: TextStyle(color: Colors.white70, fontSize: 14),
              ),
            ),
            SetMenuTitle(
              text: "TOTP Auth",
              icon: Icons.security,
              active: menuIndex == 1,
              onTap: () => onTapMenu(1),
            ),
            SetMenuTitle(
              text: "Change Password",
              icon: Icons.password,
              active: menuIndex == 2,
              onTap: () => onTapMenu(2),
            ),
            SetMenuTitle(
              text: "Article Category",
              icon: Icons.article,
              active: menuIndex == 3,
              onTap: () => onTapMenu(3),
            ),
          ],
        ),
      ),
    );
  }
}
