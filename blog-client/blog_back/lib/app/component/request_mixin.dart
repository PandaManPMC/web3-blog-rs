import 'package:blog_back/app/global/global_static.dart';
import 'package:blog_back/app/global/global_storage.dart';
import 'package:blog_back/util/request/local_request.dart';
import 'package:blog_back/util/request/request_error.dart';
import 'package:dio/dio.dart';
import 'package:flutter/material.dart';

mixin RequestMixin {
  final LocalRequest request = LocalRequest();

  CancelToken token = CancelToken();

  Future<void> requestGet(
    String path, {
    Map? params,
    Function(dynamic)? success,
    Function(int)? error,
    Function()? complete,
  }) async {
    await doRequest(
      path,
      method: NetMethod.get,
      params: params,
      success: success,
      error: error,
      complete: complete,
    );
  }

  Future<void> requestPost(
    String path, {
    Map? params,
    Function(dynamic)? success,
    Function(int)? error,
    Function()? complete,
  }) async {
    await doRequest(
      path,
      method: NetMethod.post,
      params: params,
      success: success,
      error: error,
      complete: complete,
    );
  }

  Future<void> doRequest(
    String path, {
    NetMethod? method,
    Map? params,
    Function(dynamic)? success,
    Function(int)? error,
    Function()? complete,
  }) async {
    request.baseLink = GlobalStatic.httpBaseLink;
    request.method = method ?? NetMethod.get;
    request.path = path;
    request.params = params;
    request.cancelToken = token;
    request.success = success;
    request.error = (e) => _onHttpError(e, error);
    request.complete = complete;
    request.header
        .addEntries({"x-user-token": GlobalStorage().accessToken}.entries);
    await request.request();
  }

  void _onHttpError(RequestError error1, Function(int)? error2) {
    if (error2 != null) error2(error1.code);
    showToast(error1.tip);
  }

  void showToast(String text) {
    SnackBar snackBar = SnackBar(content: Text(text));
    ScaffoldMessenger.of(GlobalStatic.navigatorKey.currentContext!)
        .showSnackBar(snackBar);
  }
}
