import http from "@/api";
// * 获取标签列表
export const getLabelLst = async (params: any) => {
	return http.get(`/article/getLabelLst`, params);
};
// * 创建标签
export const createLabel = async (params: any) => {
	return http.post(`/article/createLabel`, params);
};
// * 删除标签
export const delLabel = async (params: any) => {
	return http.post(`/article/delLabel`, params);
};
