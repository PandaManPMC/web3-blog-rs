import { createSlice, PayloadAction } from '@reduxjs/toolkit';

interface AdvertisesLstState {
    isLoading: boolean,
    data: Array<JSON>,
}

const initialState: AdvertisesLstState = {
    isLoading: true,
    data: [],
};

export const AdvertisesLstStateSlice = createSlice({
    name: 'AdvertisesLstState',
    initialState,
    reducers: {
        setAdvertisesLstState: (state, action: PayloadAction<AdvertisesLstState>) => {
            state.isLoading = action.payload.isLoading;
            state.data = action.payload.data;
        },
    },
});


