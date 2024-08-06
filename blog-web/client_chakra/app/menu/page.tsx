import React from 'react';
import TextList from "@/app/common/TextList";
import { useToast } from '@chakra-ui/react';
import TagList from "@/app/common/TagList";
import ImageCard from "@/app/common/ImageCard ";

const handleItemClick = (item: any) => {
    console.log(`Clicked on: ${JSON.stringify(item)}`);
};

const Menu = () => {
    const items = [
        {name: 'Golang 开发', id: 1, count: 20},
        {name: 'Rust 开发', id: 2, count: 240},
        {name: 'Java 开发', id: 6, count: 230},
        {name: 'Web 开发', id: 11, count: 220},
        {name: '区块链 开发', id: 12, count: 210},
    ];

    const toast = useToast();
    const tags = [
        {name: 'Rust'},
        {name: 'Golang'},
        {name: 'Ethereum'},
        {name: 'Bitcoin'},
    ];

    const handleTagClick = (tag: any) => {
        toast({
            title: `Clicked on: ${JSON.stringify(tag)}`,
            status: 'info',
            duration: 3000,
            isClosable: true,
        });
        console.log(`Clicked on: ${JSON.stringify(tag)}`);
    };

    return (
        <div>
            <ImageCard
                imageSrc="https://avatars.githubusercontent.com/u/95899886?v=4"
                title="PMC"
                description="擅长 Golang、Java、Rust、Solidity，偶尔也玩 React，主要从事区块链相关开发。"
            />
            <TextList title="笔记本" items={items} onItemClick={handleItemClick} renderItem={(item: any) => {
                return item.name + '(' + item.count +')';
            }}/>
            <TagList title="标签" tags={tags} onTagClick={handleTagClick} renderItem={(tag: any) => {
                return tag.name + '';
            }}/>
        </div>
    );
};

export default Menu;