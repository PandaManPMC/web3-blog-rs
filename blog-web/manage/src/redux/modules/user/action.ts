import * as types from "@/redux/mutation-types";

// * setBreadcrumbList
export const setUserName = (userName: string) => ({
	type: types.SET_USERNAME,
	userName
});
