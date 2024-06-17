import 'package:dio/dio.dart';

class RequestError {
  static const int codeSuccess = 2000;
  static const int codeError = -1;
  static const int codeCancel = -2;

  final int code;

  final String tip;

  dynamic data;

  RequestError(this.code, this.tip, [this.data]);

  @override
  String toString() => "code：$code || msg：$tip";

  factory RequestError.create(response) {
    if (response is DioException) {
      switch (response.type) {
        case DioExceptionType.connectionTimeout:
          return RequestError(codeError, '网络连接超时，请检查网络是否异常');
        case DioExceptionType.sendTimeout:
          return RequestError(codeError, '网络连接超时，请检查网络是否异常');
        case DioExceptionType.receiveTimeout:
          return RequestError(codeError, '网络连接超时，请检查网络是否异常');
        case DioExceptionType.badCertificate:
          return RequestError(codeError, '网络连接认证失败');
        case DioExceptionType.badResponse:
          String errorData = response.response?.data ?? "";
          return RequestError(response.response?.statusCode ?? codeError,
              errorData.isEmpty ? response.error.toString() : errorData);
        case DioExceptionType.cancel:
          return RequestError(codeCancel, '用户取消');
        case DioExceptionType.connectionError:
          return RequestError(codeError, '网络连接错误');
        case DioExceptionType.unknown:
          return RequestError(codeError, '发生错误，原因未知');
        default:
          return RequestError(codeError, '发生错误，原因未知');
      }
    } else {
      try {
        return RequestError(
            response["code"], response["tip"], response["data"]);
      } catch (e) {
        return RequestError(codeError, '请求解析异常');
      }
    }
  }
}
