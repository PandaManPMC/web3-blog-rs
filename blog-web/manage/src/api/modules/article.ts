import http from "@/api";
// * 获取文章列表
export const getArticleLst = async (params: any) => {
	return http.get(`/article/getArticleLst`, params);
};
// * 新增文章
export const articPublish = async (params: any) => {
	return http.post(`/article/publish`, params);
};
// * 编辑文章
export const changeArticle = async (params: any) => {
	return http.post(`/article/changeArticle`, params);
};

// * 获取文章类型列表
export const getClassesLst = async (params: any) => {
	return http.get(`/article/getClassesLst`, params);
};

// * 新增文章类型
export const createClasses = async (params: any) => {
	return http.post(`/article/createClasses`, params);
};

// * 删除文章类型
export const delClasses = async (params: any) => {
	return http.post(`/article/delClasses`, params);
};

// * 获取文章标签列表
export const getArticleLabelLst = async (params: any) => {
	return http.get(`/article/getArticleLabelLst`, params);
};

// * 更新文章标签
export const changeArticleLabel = async (params: any) => {
	return http.post(`/article/changeArticleLabel`, params);
};
