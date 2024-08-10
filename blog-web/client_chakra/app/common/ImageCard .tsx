"use client"
import React from 'react';
import { Box, Image, Text } from '@chakra-ui/react';

const ImageCard = ({ imageSrc, title, description }: { imageSrc: string, title: string, description: string }) => {
    return (
        <Box p={5} textAlign="center">
            <Image
                src={imageSrc}
                alt={title}
                maxW="80px"
                mx="auto"
                mb={4}
                objectFit="cover"
                borderRadius="md"
            />
            <Text fontSize="xl" fontWeight="bold" mb={2}>
                {title}
            </Text>
            <Text noOfLines={3}>
                {description}
            </Text>
        </Box>
    );
};

export default ImageCard;
