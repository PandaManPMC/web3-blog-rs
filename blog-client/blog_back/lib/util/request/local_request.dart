import 'dart:convert';

import 'package:dio/dio.dart';
import 'package:flutter/foundation.dart';

import 'request_error.dart';

enum NetMethod { get, post }

extension NetMethodExtension on NetMethod {
  String get value => ['GET', 'POST'][index];
}

class LocalRequest {
  final dio = Dio();

  void initialize() {
    dio.options.connectTimeout = const Duration(seconds: 10);
    dio.options.receiveTimeout = const Duration(seconds: 10);
    dio.options.sendTimeout = const Duration(seconds: 10);
  }

  set baseLink(String value) => dio.options.baseUrl = value;

  NetMethod _method = NetMethod.get;
  set method(NetMethod value) {
    _method = value;
    dio.options.method = _method.value;
  }

  Map<String, dynamic> get header => dio.options.headers;
  set header(value) => dio.options.headers = value;

  String _path = "";
  set path(String value) => _path = value;

  Object? _params;
  set params(Object? value) => _params = value;

  CancelToken? cancelToken;

  Function(dynamic)? _success;
  set success(Function(dynamic)? value) => _success = value;

  Function(RequestError)? _error;
  set error(Function(RequestError)? value) => _error = value;

  Function()? _complete;
  set complete(Function()? value) => _complete = value;

  Future<void> request() async {
    try {
      Map<String, dynamic>? query =
          _method == NetMethod.get ? (_params as Map<String, dynamic>?) : null;
      var data = _method == NetMethod.get ? null : _params;

      var result = await dio.request(_path,
          queryParameters: query, data: data, cancelToken: cancelToken);
      _onResponseCall(result.data);
    } catch (err) {
      _onResponseCall(err);
    } finally {
      if (_complete != null) _complete!();
    }
  }

  void _onResponseCall(data) {
    RequestError result = RequestError.create(data);
    if (result.code == RequestError.codeSuccess) {
      if (_success != null) _success!(result.data);
      debugPrint("\x1B[32m success ------------ \x1B[0m");
      debugPrint("\x1B[32m Link = ${dio.options.baseUrl}$_path \x1B[0m");
      debugPrint("\x1B[32m Header = ${dio.options.headers} \x1B[0m");
      debugPrint("\x1B[32m Param = $_params \x1B[0m");
      debugPrint("\x1B[32m Data = ${result.data} \x1B[0m");
      debugPrint("\x1B[32m success ------------ \x1B[0m");
    } else {
      if (result.code != RequestError.codeCancel) {
        if (_error != null) _error!(result);
      }
      debugPrint("\x1B[31m error ------------ \x1B[0m");
      debugPrint("\x1B[31m Link = ${dio.options.baseUrl}$_path \x1B[0m");
      debugPrint("\x1B[31m Header = ${dio.options.headers} \x1B[0m");
      debugPrint("\x1B[31m Param = $_params \x1B[0m");
      debugPrint("\x1B[31m Error = $result \x1B[0m");
      debugPrint("\x1B[31m error ------------ \x1B[0m");
    }
  }
}
