import { createSlice, PayloadAction } from '@reduxjs/toolkit';

interface ClassesLstState {
    isLoading: boolean,
    data: Array<JSON>,
}

const initialState: ClassesLstState = {
    isLoading: true,
    data: [],
};

export const ClassesLstStateSlice = createSlice({
    name: 'ClassesLstState',
    initialState,
    reducers: {
        setClassesLstState: (state, action: PayloadAction<ClassesLstState>) => {
            state.isLoading = action.payload.isLoading;
            state.data = action.payload.data;
        },
    },
});


