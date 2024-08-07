'use client'

import React from 'react';
import { Box, Image, Text, LinkBox, LinkOverlay, VStack  } from '@chakra-ui/react';

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
    return (
        <Box p={5}>
            <VStack spacing={4}>
                <ClickableCard
                    imageSrc="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcT2vAhoyOagozq-H6S1Qbp9a0vVJsmm4zlyqQ&s"
                    text="OKX 交易所注册，赠送新手礼包"
                    link="https://cnouyi.studio/join/9386831"
                />
                <ClickableCard
                    imageSrc="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSZniO0SS3WO7PJyIDStSI6rF9NV-iX2goRHA&s"
                    text="Gate.io 交易所注册，返佣10%"
                    link="https://www.gate.io/signup/UVMQBFEJ?ref_type=103"
                />
            </VStack>
        </Box>
    )
        ;
};

export default Advertise;