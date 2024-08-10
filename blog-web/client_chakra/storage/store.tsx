// store.ts
import { configureStore, createSlice, PayloadAction } from '@reduxjs/toolkit';

interface AuthorState {
    penName: String,
    profile: String,
    introduce: String,
    mkFooter: String,
}

const initialState: AuthorState = {
    penName: "",
    profile: "",
    introduce: "",
    mkFooter: "",
};

const authorStateSlice = createSlice({
    name: 'Author',
    initialState,
    reducers: {
        setAuthorState: (state, action: PayloadAction<AuthorState>) => {
            state.penName = action.payload.penName;
            state.profile = action.payload.profile;
            state.introduce = action.payload.introduce;
            state.mkFooter = action.payload.mkFooter;
        },
    },
});

export const { setAuthorState } = authorStateSlice.actions;
const authorReducer = authorStateSlice.reducer;

export const store = configureStore({
    reducer: {
        author: authorReducer,
    },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
