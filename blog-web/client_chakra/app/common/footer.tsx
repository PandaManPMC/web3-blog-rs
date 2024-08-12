'use client'

import {
    Flex,
    Box,
    chakra,
    Container,
    SimpleGrid,
    Stack,
    Text,
    VisuallyHidden,
    Input,
    IconButton,
    useColorModeValue, Avatar,
} from '@chakra-ui/react'
import React, { ReactNode } from 'react'
import {FaGithub, FaTwitter} from 'react-icons/fa'
import { BiMailSend } from 'react-icons/bi'
import "../globals.css";
import {useSelector} from "react-redux";
import {RootState} from "@/storage/store";


const SocialButton = ({
                          children,
                          label,
                          href,
                      }: {
    children: ReactNode
    label: string
    href: string
}) => {
    return (
        <chakra.button
            bg={useColorModeValue('blackAlpha.100', 'whiteAlpha.100')}
            rounded={'full'}
            w={8}
            h={8}
            cursor={'pointer'}
            as={'a'}
            href={href}
            display={'inline-flex'}
            alignItems={'center'}
            justifyContent={'center'}
            transition={'background 0.3s ease'}
            target="_blank"
            rel="noopener noreferrer"
            _hover={{
                bg: useColorModeValue('blackAlpha.200', 'whiteAlpha.200'),
            }}>
            <VisuallyHidden>{label}</VisuallyHidden>
            {children}
        </chakra.button>
    )
}

const ListHeader = ({children}: { children: ReactNode }) => {
    return (
        <Text fontWeight={'500'} fontSize={'lg'} mb={2}>
            {children}
        </Text>
    )
}

export default function LargeWithNewsletter() {
    const author = useSelector((state: RootState) => state.author);

    const Logo = (props: any) => {

        return (
            <>
                <Flex justify="space-between" align="center" p={4}>
                    <Box>
                        <Avatar size='lg' name='Logo' src={author.profile} />{' '}
                    </Box>
                    <Box>
                        <Text fontSize="lg">{author.penName}</Text>
                    </Box>
                    <Box>
                    </Box>
                    <Box>
                    </Box>
                    <Box>
                    </Box>
                </Flex>
            </>
        )
    }

    return (
        <>
            <Box
                bg={useColorModeValue('gray.50', 'gray.900')}
                color={useColorModeValue('gray.700', 'gray.200')}>
                <Container as={Stack} maxW={'6xl'} py={10}>
                    <SimpleGrid
                        templateColumns={{sm: '1fr 1fr', md: '2fr 1fr 1fr 2fr'}}
                        spacing={8}>
                        <Stack spacing={6}>
                            <Box>
                                <Logo color={useColorModeValue('gray.700', 'white')}/>
                            </Box>
                            <Text fontSize={'sm'}>© 2023 PandaManCoin. All rights reserved</Text>
                            <Stack direction={'row'} spacing={6}>
                                <SocialButton label={'Twitter'} href={'https://x.com/pandamancoin'}>
                                    <FaTwitter/>
                                </SocialButton>
                                <SocialButton label={'Github'} href={'https://github.com/PandaManPMC'}>
                                    <FaGithub/>
                                </SocialButton>
                            </Stack>
                        </Stack>
                        <Stack align={'flex-start'}>
                            <ListHeader>Company</ListHeader>
                            <Box as="a" href={'https://github.com/PandaManPMC/web3-blog-rs'} target="_blank">
                                About us
                            </Box>
                            <Box as="a" href={'#'}>
                                Blog
                            </Box>
                            <Box as="a" href={'#'}>
                                {author.contactMail}
                            </Box>
                        </Stack>
                        <Stack align={'flex-start'}>
                            <ListHeader>Support</ListHeader>
                            <Box as="a" href={'#'}>
                                Privacy Policy
                            </Box>
                            <Box as="a" href={'#'}>
                                Satus
                            </Box>
                        </Stack>
                        <Stack align={'flex-start'}>
                            <ListHeader>Stay up to date</ListHeader>
                            <Stack direction={'row'}>
                                <Input
                                    placeholder={'Your email address'}
                                    bg={useColorModeValue('blackAlpha.100', 'whiteAlpha.100')}
                                    border={0}
                                    _focus={{
                                        bg: 'whiteAlpha.300',
                                    }}/>
                                <IconButton
                                    bg={useColorModeValue('green.400', 'green.800')}
                                    color={useColorModeValue('white', 'gray.800')}
                                    _hover={{
                                        bg: 'green.600',
                                    }}
                                    aria-label="Subscribe"
                                    icon={<BiMailSend/>}/>
                            </Stack>
                        </Stack>
                    </SimpleGrid>
                </Container>
            </Box>
        </>
    )
}