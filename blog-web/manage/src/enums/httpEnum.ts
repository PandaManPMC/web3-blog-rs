// * 请求枚举配置
/**
 * @description：请求配置
 */
// // 操作码-成功
// pub const CODE_OK: i16 = 2000;
// /// 备选-特殊业务情况-重定向
// pub const CODE_REDIRECT: i16 = 2001;
// /// 没有权限
// pub const CODE_PERMISSION_DENIED: i16 = 2002;
// /// 备选-特殊业务情况-重定向2
// pub const CODE_REDIRECT2: i16 = 2003;
// /// 操作码-系统错误
// pub const CODE_ERROR: i16 = 2004;
// /// 没有登录
// pub const CODE_NOT_LOGIN: i16 = 2008;
// /// 缺少必要参数
// pub const CODE_FAIL_PARAMS: i16 = 2010;
// /// 操作失败
// pub const CODE_FAIL: i16 = 2014;
// /// 警告,操作出现一些意外情况,结果可能不确定
// pub const CODE_WARN: i16 = 2015;
// /// 图形验证码错误
// pub const CODE_WRONG_CAPTCHA: i16 = 2021;
// /// 验证码错误（手机、邮箱等）
// pub const CODE_WRONG_CODE: i16 = 2022;
// /// 验证通行凭证过期
// pub const CODE_GATE_EXPIRES: i16 = 2023;
// /// 操作频繁，限控
// pub const CODE_FREQUENT: i16 = 2024;
// /// 系统维护中
// pub const CODE_MAINTAIN: i16 = 2025;
// /// 签名校验错误
// pub const CODE_SIGNATURE_FAIL: i16 = 2025;
export enum ResultEnum {
	SUCCESS = 2000,
	CODE_ERROR = 2004,
	CODE_NOT_LOGIN = 2008,
	CODE_FAIL_PARAMS = 2010,
	ERROR = 2014,
	CODE_WARN = 2015
}

/**
 * @description：请求方法
 */
export enum RequestEnum {
	GET = "GET",
	POST = "POST",
	PATCH = "PATCH",
	PUT = "PUT",
	DELETE = "DELETE"
}

/**
 * @description：常用的contentTyp类型
 */
export enum ContentTypeEnum {
	// json
	JSON = "application/json;charset=UTF-8",
	// text
	TEXT = "text/plain;charset=UTF-8",
	// form-data 一般配合qs
	FORM_URLENCODED = "application/x-www-form-urlencoded;charset=UTF-8",
	// form-data 上传
	FORM_DATA = "multipart/form-data;charset=UTF-8"
}
