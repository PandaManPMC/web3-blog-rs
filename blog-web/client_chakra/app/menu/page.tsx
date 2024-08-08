'use client'

import React, { useEffect, useState } from 'react';
import TextList from "@/app/common/TextList";
import TagList from "@/app/common/TagList";
import ImageCard from "@/app/common/ImageCard ";
import {get, useGetWrap} from "@/tool/http";
import {useErrToast, useInfoToast} from "@/tool/ui";

const handleItemClick = (item: any) => {
    console.log(`Clicked on: ${JSON.stringify(item)}`);
};
var counter = 0;
const Menu = () => {
    const errToast = useErrToast();
    const infoToast = useInfoToast();
    const getWrap = useGetWrap();

    const [classesLst, setClassesLst] = useState([]);
    const [classesLstLoading, setClassesLstLoading] = useState(true);
    const [labLst, setLabLst] = useState([]);
    const [labLstLoading, setLabLstLoading] = useState(true);

    counter++;
    console.log("menu" + counter);

    useEffect(() => {
        console.log("useEffect");
        getClassesLst();
        getLabLst();
    }, []);

    const handleTagClick = (tag: any) => {
        console.log(`Clicked on: ${JSON.stringify(tag)}`);
        infoToast(`Clicked on: ${JSON.stringify(tag)}`);
    };

    const getClassesLst = async () => {
        let data,error;
        try {
            data = await get('/article/classes');
        } catch (err) {
            error = err;
        } finally {
            setClassesLstLoading(false);
        }
        console.log(data);
        if (error) {
            console.log(error);
            errToast(`http error`, JSON.stringify(error));
            return;
        }

        if (2000 != data.code) {
            errToast(`http ${data.code}`, data.tip);
            return;
        }

        setClassesLst(data.data);
    }

    const getLabLst = async () => {
        let data;
        try {
            data = await getWrap('/article/labels');
        } catch (err) {
            return;
        }finally {
            setLabLstLoading(false);
        }
        console.log(data);

        if (2000 != data.code) {
            return;
        }

        setLabLst(data.data);
    }

    return (
        <div>
            <ImageCard
                imageSrc="https://avatars.githubusercontent.com/u/95899886?v=4"
                title="PMC"
                description="擅长 Golang、Java、Rust、Solidity，偶尔也玩 React，主要从事区块链行业。"
            />
            <TextList title="笔记本" items={classesLst} isLoading={classesLstLoading} onItemClick={handleItemClick} renderItem={(item: any) => {
                return item.classesName;
            }}/>
            <TagList title="标签" tags={labLst} isLoading={labLstLoading} onTagClick={handleTagClick} renderItem={(tag: any) => {
                return tag.labelName;
            }}/>
        </div>
    );
};

export default Menu;