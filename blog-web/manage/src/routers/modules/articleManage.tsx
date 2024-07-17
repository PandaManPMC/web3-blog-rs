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
				path: "/articleManage/useHooks",
				element: lazyLoad(React.lazy(() => import("@/views/proTable/useHooks/index"))),
				meta: {
					requiresAuth: true,
					title: "标签管理",
					icon: "AppstoreOutlined",
					key: "useHooks"
				}
			},
			{
				path: "/articleManage/useComponent",
				element: lazyLoad(React.lazy(() => import("@/views/proTable/useComponent/index"))),
				meta: {
					requiresAuth: true,
					title: "文章管理",
					key: "useComponent",
					icon: "AppstoreOutlined"
				}
			}
		]
	}
];

export default proTableRouter;
