"use client"
import { useSelector, useDispatch } from 'react-redux';
import {
    RootState,
    AppDispatch,
    setAuthorState,
    setClassesLstState,
    setLabelLstState,
    setAdvertisesLstState
} from '@/storage/store';
import {get, useGetWrap} from "@/tool/http";
import {useEffect} from "react";
import {usePathname} from "next/navigation";

const InitData = () => {
    const getWrap = useGetWrap();
    const dispatch = useDispatch<AppDispatch>();
    // const author = useSelector((state: RootState) => state.author); // 使用
    const isHomePage = usePathname() == "/";

    useEffect(() => {
        getAuthorInfo();
        getAdvertisesLst();

        if (isHomePage) {
            getClassesLst();
            getLabLst();
        }

    }, []);

    const getAuthorInfo = async () => {

        let data;
        try {
            data = await getWrap('/author/info');
        } catch (err) {
        }
        if (2000 != data.code) {
            return;
        }
        dispatch(setAuthorState(data.data));
    }

    const getClassesLst = async () => {
        let data;
        try {
            data = await getWrap('/article/classes');
        } catch (err) {
        }

        if (2000 != data.code) {
            dispatch(setClassesLstState({isLoading: false, data: []}));
            return;
        }
        dispatch(setClassesLstState({isLoading: false, data: data.data}));
    }

    const getLabLst = async () => {
        let data;
        try {
            data = await getWrap('/article/labels');
        } catch (err) {
            return;
        }

        if (2000 != data.code) {
            dispatch(setClassesLstState({isLoading: false, data: []}));
            return;
        }

        dispatch(setLabelLstState({isLoading: false, data: data.data}));
    }

    const getAdvertisesLst = async () => {

        let data;
        try {
            data = await getWrap('/advertise/list');
        } catch (err) {
        }

        if (2000 != data.code) {
            dispatch(setAdvertisesLstState({isLoading: false, data: []}));
            return;
        }
        dispatch(setAdvertisesLstState({isLoading: false, data: data.data}));
    }

    return (
        <></>
    );
};

export default InitData;
