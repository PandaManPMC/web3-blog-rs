import { Navigate, useRoutes } from "react-router-dom";
import { RouteObject } from "@/routers/interface";
import Login from "@/views/login/index";
import lazyLoad from "@/routers/utils/lazyLoad";
import React from "react";

// * 导入所有router
const metaRouters = import.meta.globEager("./modules/*.tsx");

// * 处理路由
export const routerArray: RouteObject[] = [];
Object.keys(metaRouters).forEach(item => {
	Object.keys(metaRouters[item]).forEach((key: any) => {
		routerArray.push(...metaRouters[item][key]);
	});
});

export const rootRouter: RouteObject[] = [
	{
		path: "/",
		element: <Navigate to="/login" />
	},
	{
		path: "/login",
		element: <Login />,
		meta: {
			requiresAuth: false,
			title: "登录页",
			key: "login"
		}
	},
	...routerArray,
	{
		path: "/404",
		element: lazyLoad(React.lazy(() => import("@/components/ErrorMessage/404"))),
		meta: {
			requiresAuth: false,
			title: "404页面",
			key: "404"
		}
	},
	{
		path: "*",
		element: <Navigate to="/404" />
	}
];

const Router = () => {
	const routes = useRoutes(rootRouter);
	return routes;
};

export default Router;
