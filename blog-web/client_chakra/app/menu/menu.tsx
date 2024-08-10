'use client'

import React, { useEffect, useState } from 'react';
import TextList from "@/app/common/TextList";
import TagList from "@/app/common/TagList";
import ImageCard from "@/app/common/ImageCard ";
import {get, useGetWrap} from "@/tool/http";
import {useErrToast, useInfoToast} from "@/tool/ui";
import {useSelector} from "react-redux";
import {RootState} from "@/storage/store";

let selectedQuery = {idBlogLabel: 0, idBlogClasses: 0};
let counter = 0;

const Menu = ({onMenuSelectedQuery}: {onMenuSelectedQuery: any}) => {
    const errToast = useErrToast();
    const getWrap = useGetWrap();
    const author = useSelector((state: RootState) => state.author);

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

    const handleItemClick = (item: any) => {
        // console.log(`Clicked on: ${JSON.stringify(item)}`);
        if (selectedQuery.idBlogClasses == item.id){
            selectedQuery.idBlogClasses = 0;
        }else{
            selectedQuery.idBlogClasses = item.id;
        }
        onMenuSelectedQuery(selectedQuery);
    };

    const handleTagClick = (tag: any) => {
        // console.log(`Clicked on: ${JSON.stringify(tag)}`);
        if (selectedQuery.idBlogLabel == tag.id){
            selectedQuery.idBlogLabel = 0;
        }else{
            selectedQuery.idBlogLabel = tag.id;
        }
        onMenuSelectedQuery(selectedQuery);
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

        if (2000 != data.code) {
            return;
        }

        setLabLst(data.data);
    }

    return (
        <div>
            <ImageCard
                // @ts-ignore
                imageSrc={author.profile}
                // @ts-ignore
                title={author.penName}
                // @ts-ignore
                description={author.introduce}
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