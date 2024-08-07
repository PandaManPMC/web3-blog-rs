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
			}
			// {
			// 	path: "/articleManage/articleLabel",
			// 	element: lazyLoad(React.lazy(() => import("@/views/articleCenter/articleLabel/index"))),
			// 	meta: {
			// 		requiresAuth: true,
			// 		title: "文章关联标签列表",
			// 		key: "articleLabel",
			// 		icon: "AppstoreOutlined"
			// 	}
			// }
		]
	},
	{
		element: <LayoutIndex />,
		meta: {
			title: "文章中心",
			icon: "FileTextOutlined",
			hidden: true
		},
		path: "/articleView",
		sort: 2,
		children: [
			{
				path: "/articleView/index",
				element: lazyLoad(React.lazy(() => import("@/views/articleCenter/articleView/index"))),
				meta: {
					requiresAuth: false,
					title: "文章预览",
					key: "articleList",
					icon: "AppstoreOutlined"
				}
			}
		]
	}
];

export default proTableRouter;
