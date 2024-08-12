// store.ts
import { createSlice, PayloadAction } from '@reduxjs/toolkit';

interface AuthorState {
    penName: string,
    profile: string,
    introduce: string,
    mkFooter: string,
    contactMail: string,
}

const initialState: AuthorState = {
    penName: "",
    profile: "",
    introduce: "",
    mkFooter: "",
    contactMail: "",
};

export const  authorStateSlice = createSlice({
    name: 'Author',
    initialState,
    reducers: {
        setAuthorState: (state, action: PayloadAction<AuthorState>) => {
            state.penName = action.payload.penName;
            state.profile = action.payload.profile;
            state.introduce = action.payload.introduce;
            state.mkFooter = action.payload.mkFooter;
            state.contactMail = action.payload.contactMail;
        },
    },
});


