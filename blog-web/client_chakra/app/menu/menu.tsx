'use client'

import React, { useEffect, useState, useRef } from 'react';
import TextList from "@/app/common/TextList";
import TagList from "@/app/common/TagList";
import ImageCard from "@/app/common/ImageCard ";
import {useDispatch, useSelector} from "react-redux";
import {AppDispatch, RootState, setArticleTOCState, setSelectedQueryState} from "@/storage/store";
import { usePathname } from 'next/navigation';
import 'github-markdown-css/github-markdown.css';
import './menu.css'

let selectedQuery = {idBlogLabel: 0, idBlogClasses: 0};

const Menu = () => {
    const author = useSelector((state: RootState) => state.author);
    const dispatch = useDispatch<AppDispatch>();

    const cla = useSelector((state: RootState) => state.ClassesLstStateSliceReducer);
    let classesLstLoading = cla.isLoading;
    let classesLst = cla.data;

    const lab = useSelector((state: RootState) => state.LabelLstStateSliceReducer);
    let labLstLoading = lab.isLoading;
    let labLst = lab.data;

    const isHomePage = usePathname() == "/";
    const isArticlePage = usePathname() == "/article/";

    const toc = useSelector((state: RootState) => state.ArticleTOCStateSliceReducer).TOC;
    const tocContainerRef = useRef(null);
    const [tocWidth, setTocWidth] = useState('auto');
    const [isFixed, setIsFixed] = useState(false);

    useEffect(() => {
        if (tocContainerRef.current) {
            // 记录固定前的宽度
            if (!isFixed) {
                // @ts-ignore
                const width = tocContainerRef.current.offsetWidth;
                setTocWidth(`${width}px`);
            }
        }
    }, [isFixed]);

    const handleTocClick = (slug: any) => (event: any) => {
        dispatch(setArticleTOCState({TOC: [], SelectId: slug}));
        event.preventDefault();
    };

    const handleItemClick = (item: any) => {
        // console.log(`Clicked on: ${JSON.stringify(item)}`);
        if (selectedQuery.idBlogClasses == item.id){
            selectedQuery.idBlogClasses = 0;
        }else{
            selectedQuery.idBlogClasses = item.id;
        }
        dispatch(setSelectedQueryState(selectedQuery));
    };

    const handleTagClick = (tag: any) => {
        // console.log(`Clicked on: ${JSON.stringify(tag)}`);
        if (selectedQuery.idBlogLabel == tag.id){
            selectedQuery.idBlogLabel = 0;
        }else{
            selectedQuery.idBlogLabel = tag.id;
        }
        dispatch(setSelectedQueryState(selectedQuery));
    };


    useEffect(() => {
        const handleScroll = () => {
            const scrollPosition = window.scrollY;
            const tocElement = document.getElementById('toc-container');
            if (tocElement) {
                const tocTop = tocElement.offsetTop;
                if (scrollPosition > tocTop) {
                    setIsFixed(true);
                } else {
                    setIsFixed(false);
                }
            }
        };

        window.addEventListener('scroll', handleScroll);
        return () => {
            window.removeEventListener('scroll', handleScroll);
        };
    }, []);


    return (
        <div style={{width: '100%'}}>
            <ImageCard
                // @ts-ignore
                imageSrc={author.profile}
                // @ts-ignore
                title={author.penName}
                // @ts-ignore
                description={author.introduce}
            />
            {isHomePage ? (
                <>
                    <TextList title="笔记本" items={classesLst} isLoading={classesLstLoading} onItemClick={handleItemClick} renderItem={(item: any) => {
                        return item.classesName;
                    }}/>
                    <TagList title="标签" tags={labLst} isLoading={labLstLoading} onTagClick={handleTagClick} renderItem={(tag: any) => {
                        return tag.labelName;
                    }}/>
                </>
            ): null}
            {isArticlePage ? (
                // <div className="markdown-body" style={{backgroundColor: 'transparent'}}>
                //     <h2 style={{textAlign: 'center'}}>目录</h2>
                //     <ul>
                //         {toc.map((item, index) => (
                //             <li key={index} style={{marginLeft: (item.level - 1) * 20}}>
                //                 <a href={`#${item.slug}`} onClick={handleTocClick(item.slug)}>
                //                     {item.text}
                //                 </a>
                //             </li>
                //         ))}
                //     </ul>
                // </div>
                <div
                    id="toc-container"
                    className={`markdown-body ${isFixed ? 'fixed-toc' : ''}`}
                    style={{backgroundColor: 'transparent', width: isFixed ? tocWidth : 'auto'}}
                    ref={tocContainerRef}
                >
                    <h2 style={{textAlign: 'center'}}>目录</h2>
                    <ul>
                        {toc.map((item, index) => (
                            <li key={index} style={{marginLeft: (item.level - 1) * 20}}>
                                <a href={`#${item.slug}`} onClick={handleTocClick(item.slug)}>
                                    {item.text}
                                </a>
                            </li>
                        ))}
                    </ul>
                </div>
            ) : null}
        </div>
    );
};

export default Menu;