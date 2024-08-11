// store.ts
import { createSlice, PayloadAction } from '@reduxjs/toolkit';

interface SelectedQueryState {
    idBlogLabel: number,
    idBlogClasses: number,
}

const initialState: SelectedQueryState = {
    idBlogLabel: -1,
    idBlogClasses: -1,
};

export const selectedQueryStateSlice = createSlice({
    name: 'SelectedQuery',
    initialState,
    reducers: {
        setSelectedQueryState: (state, action: PayloadAction<SelectedQueryState>) => {
            state.idBlogLabel = action.payload.idBlogLabel;
            state.idBlogClasses = action.payload.idBlogClasses;
        },
    },
});


