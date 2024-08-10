"use client"
import { useSelector, useDispatch } from 'react-redux';
import { RootState, AppDispatch, setAuthorState } from '@/storage/store';
import {useGetWrap} from "@/tool/http";
import {useEffect} from "react";

const InitData = () => {
    const getWrap = useGetWrap();
    const dispatch = useDispatch<AppDispatch>();
    // const author = useSelector((state: RootState) => state.author); // 使用

    useEffect(() => {
        getAuthorInfo();
    }, []);

    const getAuthorInfo = async () => {

        let data;
        try {
            data = await getWrap('/author/info');
        } catch (err) {
            return;
        }
        if (2000 != data.code) {
            return;
        }
        dispatch(setAuthorState(data.data));
    }

    return (
        <div></div>
    );
};

export default InitData;
