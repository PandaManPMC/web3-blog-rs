import { createSlice, PayloadAction } from '@reduxjs/toolkit';

interface LabelLstState {
    isLoading: boolean,
    data: Array<JSON>,
}

const initialState: LabelLstState = {
    isLoading: true,
    data: [],
};

export const LabelLstStateSlice = createSlice({
    name: 'LabelLstState',
    initialState,
    reducers: {
        setLabelLstState: (state, action: PayloadAction<LabelLstState>) => {
            state.isLoading = action.payload.isLoading;
            state.data = action.payload.data;
        },
    },
});


