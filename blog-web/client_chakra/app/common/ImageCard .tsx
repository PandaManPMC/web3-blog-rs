import React from 'react';
import { Box, Image, Text } from '@chakra-ui/react';

const ImageCard = ({ imageSrc, title, description }: { imageSrc: string, title: string, description: string }) => {
    return (
        <Box p={5} borderWidth="1px" borderRadius="lg" overflow="hidden">
            <Box float="left" mr={4} mb={2}>
                <Image
                    src={imageSrc}
                    alt={title}
                    boxSize="150px"
                    objectFit="cover"
                    borderRadius="md"
                />
            </Box>
            <Text fontSize="xl" fontWeight="bold" mb={2}>
                {title}
            </Text>
            <Text>
                {description}
            </Text>
            <Box as="div" sx={{ clear: 'both' }} />
        </Box>
    );
};

export default ImageCard;
