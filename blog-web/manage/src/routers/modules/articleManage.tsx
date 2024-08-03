import React from "react";
import lazyLoad from "@/routers/utils/lazyLoad";
import { LayoutIndex } from "@/routers/constant";
import { RouteObject } from "@/routers/interface";

// 超级表格模块
const proTableRouter: Array<RouteObject> = [
	{
		element: <LayoutIndex />,
		meta: {
			title: "文章中心",
			icon: "FileTextOutlined"
		},
		path: "/articleManage",
		sort: 2,
		children: [
			{
				path: "/articleManage/articleList",
				element: lazyLoad(React.lazy(() => import("@/views/articleCenter/articleList/index"))),
				meta: {
					requiresAuth: true,
					title: "文章列表",
					key: "articleList",
					icon: "AppstoreOutlined"
				}
			},
			{
				path: "/articleManage/classesList",
				element: lazyLoad(React.lazy(() => import("@/views/articleCenter/classsesList/index"))),
				meta: {
					requiresAuth: true,
					title: "类型列表",
					key: "classesList",
					icon: "AppstoreOutlined"
				}
			},
			{
				path: "/articleManage/labelList",
				element: lazyLoad(React.lazy(() => import("@/views/articleCenter/labelList/index"))),
				meta: {
					requiresAuth: true,
					title: "标签列表",
					key: "labelList",
					icon: "AppstoreOutlined"
				}
			},
			{
				path: "/articleManage/articleContent",
				element: lazyLoad(React.lazy(() => import("@/views/articleCenter/articleList/articleContent"))),
				meta: {
					requiresAuth: true,
					title: "文章内容",
					key: "articleContent",
					icon: "AppstoreOutlined"
				}
			},
			{
				path: "/articleManage/articleEdit",
				element: lazyLoad(React.lazy(() => import("@/views/articleCenter/articleList/articleEdit"))),
				meta: {
					requiresAuth: true,
					title: "编辑文章",
					key: "articleEdit",
					icon: "AppstoreOutlined"
				}
			}
		]
	}
];

export default proTableRouter;
