// 当前查看的文章的目录
import { createSlice, PayloadAction } from '@reduxjs/toolkit';

interface ArticleTOCState {
    TOC: Array<{level: any; text: any; slug: string;}>,
    SelectId: string,
}

const initialState: ArticleTOCState = {
    TOC: [],
    SelectId: "",
};

export const  ArticleTOCStateSlice = createSlice({
    name: 'ArticleTOC',
    initialState,
    reducers: {
        setArticleTOCState: (state, action: PayloadAction<ArticleTOCState>) => {
            if (0 != action.payload.TOC.length) {
                state.TOC = action.payload.TOC;
            }
            if ("" != action.payload.SelectId) {
                state.SelectId = action.payload.SelectId;
            }
        },
    },
});


