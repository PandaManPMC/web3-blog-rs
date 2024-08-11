// store.ts
import { configureStore } from '@reduxjs/toolkit';
import {selectedQueryStateSlice} from "@/storage/SelectedQuery";
import {authorStateSlice} from "@/storage/AuthorState";
import {ClassesLstStateSlice} from "@/storage/ClassesLst";
import {LabelLstStateSlice} from "@/storage/LabelLst";
import {AdvertisesLstStateSlice} from "@/storage/AdvertisesLst";
import {ArticleTOCStateSlice} from "@/storage/ArticleTOC";

export const { setAuthorState } = authorStateSlice.actions;
const authorReducer = authorStateSlice.reducer;

export const { setSelectedQueryState } = selectedQueryStateSlice.actions;
const selectedQueryReducer = selectedQueryStateSlice.reducer;

export const { setClassesLstState } = ClassesLstStateSlice.actions;
const ClassesLstStateSliceReducer = ClassesLstStateSlice.reducer;

export const { setLabelLstState } = LabelLstStateSlice.actions;
const LabelLstStateSliceReducer = LabelLstStateSlice.reducer;

export const { setAdvertisesLstState } = AdvertisesLstStateSlice.actions;
const AdvertisesLstStateSliceReducer = AdvertisesLstStateSlice.reducer;

export const { setArticleTOCState } = ArticleTOCStateSlice.actions;
const ArticleTOCStateSliceReducer = ArticleTOCStateSlice.reducer;

export const store = configureStore({
    reducer: {
        author: authorReducer,
        selectedQuery: selectedQueryReducer,
        ClassesLstStateSliceReducer: ClassesLstStateSliceReducer,
        LabelLstStateSliceReducer: LabelLstStateSliceReducer,
        AdvertisesLstStateSliceReducer: AdvertisesLstStateSliceReducer,
        ArticleTOCStateSliceReducer: ArticleTOCStateSliceReducer,
    },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
