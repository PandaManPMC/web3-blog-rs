'use client'

import React, {useEffect, useState} from 'react';
import {Box, Image, Text, LinkBox, LinkOverlay, VStack, Button} from '@chakra-ui/react';
import {useGetWrap} from "@/tool/http";
import ArticleCard from "@/app/common/ArticleCard";

const ClickableCard = ({ imageSrc, text, link }: { imageSrc: string, text: string, link: string }) => {
    return (
        <LinkBox as="article" maxW="sm" borderWidth="1px" borderRadius="lg" overflow="hidden">
            <Image src={imageSrc} alt={text} />
            <Box p="6">
                {/*@ts-ignore*/}
                <Box d="flex" alignItems="baseline">
                    <LinkOverlay href={link} isExternal>
                        <Text mt="1" fontWeight="semibold" as="h4" lineHeight="tight">
                            {text}
                        </Text>
                    </LinkOverlay>
                </Box>
            </Box>
        </LinkBox>
    );
};

const Advertise = () => {
    const getWrap = useGetWrap();

    const [advertiseLst, setAdvertisesLst] = useState([]);
    const [advertiseLstLoading, setAdvertiseLstLoading] = useState(true);

    useEffect(() => {
        getAdvertisesLst();
    }, []);

    const getAdvertisesLst = async () => {

        let data;
        try {
            data = await getWrap('/advertise/list');
        } catch (err) {
            return;
        }finally {
            setAdvertiseLstLoading(false);
        }
        console.log(data);

        if (2000 != data.code) {
            return;
        }

        // @ts-ignore
        setAdvertisesLst([...data.data]);
    }

    return (
        <Box p={5}>
            {advertiseLstLoading ? (
                <Box display="flex" justifyContent="center" mt={4}>
                    <Button isLoading={advertiseLstLoading} loadingText="加载中" colorScheme="gray" ></Button>
                </Box>
                ): null }
            <VStack spacing={4}>
                {advertiseLst.map((obj, index) => (
                    <ClickableCard
                        key={index}
                        // @ts-ignore
                        imageSrc={obj.img1}
                        // @ts-ignore
                        text={obj.title}
                        // @ts-ignore
                        link={obj.linkAdvertise}
                    />
                ))}
            </VStack>
        </Box>
    )
        ;
};

export default Advertise;