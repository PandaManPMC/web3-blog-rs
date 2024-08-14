import { AnyAction } from "redux";
import { UserState } from "@/redux/interface";
import produce from "immer";
import * as types from "@/redux/mutation-types";

const userState: UserState = {
	userName: ""
};

// breadcrumb reducer
const userinfo = (state: UserState = userState, action: AnyAction) =>
	produce(state, draftState => {
		switch (action.type) {
			case types.SET_USERNAME:
				draftState.userName = action.userName;
				break;
			default:
				return draftState;
		}
	});

export default userinfo;
