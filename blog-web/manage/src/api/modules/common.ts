import http from "@/api";
// * 校验谷歌验证码
export const getVerifyReCaptchaToken = (params: any) => {
	return http.get(`/common/verifyReCaptchaToken`, params);
};
