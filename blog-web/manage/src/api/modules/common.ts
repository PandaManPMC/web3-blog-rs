import http from "@/api";
import { Login } from "@/api/interface";
// * 校验谷歌验证码
export const getVerifyReCaptchaToken = (params: any) => {
	return http.get(`/common/verifyReCaptchaToken`, params);
};

// * 上传文件
export const fileUpload = async (file: File) => {
	const formData = new FormData();
	formData.append("file", file);
	return http.post(`/common/fileUpload`, formData);
};
