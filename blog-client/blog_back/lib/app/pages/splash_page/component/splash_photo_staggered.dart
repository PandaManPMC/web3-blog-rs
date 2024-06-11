import 'package:flutter/material.dart';
import 'package:flutter_staggered_grid_view/flutter_staggered_grid_view.dart';

import 'splash_photo.dart';

class SplashPhotoStaggered extends StatelessWidget {
  const SplashPhotoStaggered({super.key});

  @override
  Widget build(BuildContext context) {
    return StaggeredGrid.count(
      crossAxisCount: 3,
      mainAxisSpacing: 8,
      crossAxisSpacing: 8,
      children: const [
        StaggeredGridTile.count(
          crossAxisCellCount: 1,
          mainAxisCellCount: 1.5,
          child: SplashPhoto(photo: "images/photo1.png"),
        ),
        StaggeredGridTile.count(
          crossAxisCellCount: 2,
          mainAxisCellCount: 1.5,
          child: SplashPhoto(photo: "images/photo2.png"),
        ),
        StaggeredGridTile.count(
          crossAxisCellCount: 2,
          mainAxisCellCount: 1.5,
          child: SplashPhoto(photo: "images/photo3.png"),
        ),
        StaggeredGridTile.count(
          crossAxisCellCount: 1,
          mainAxisCellCount: 1.5,
          child: SplashPhoto(photo: "images/photo4.png"),
        ),
      ],
    );
  }
}
