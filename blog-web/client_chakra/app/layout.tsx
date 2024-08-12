"use client";

import React, {useEffect, useState} from 'react';
import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import LargeWithNewsletter from "./common/footer";
import {GlobalStateProvider} from "@/context/GlobalState";
import {Box, ChakraProvider, Grid} from "@chakra-ui/react";
import { extendTheme } from '@chakra-ui/react';
const inter = Inter({ subsets: ["latin"] });
import Advertise from "./advertise/Advertise";
import Menu from "./menu/menu";
import ScrollToTopButton from "@/app/common/ScrollToTopButton";
import { Provider } from 'react-redux';
import { store } from '@/storage/store';
import InitData from "@/app/common/InitData";
import {MetaMaskProvider} from "@metamask/sdk-react"

// 自定义主题
const theme = extendTheme({
    colors: {
        primary: {
            50: '#e3f9e5',
            100: '#c1eac5',
            200: '#a3d9a5',
            300: '#7bc47f',
            400: '#57ae5b',
            500: '#3f9142',
            600: '#2f8132',
            700: '#207227',
            800: '#0e5814',
            900: '#05400a',
        },
    },
    fonts: {
        heading: 'Arial, sans-serif',
        body: 'Arial, sans-serif',
    },
    styles: {
        global: {
            'html, body': {
                color: 'gray.800',
                background: 'gray.100',
            },
        },
    },
});

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
    return (
      <html lang="zh">
      <head>
          <title>Next Template</title>
          <meta name="viewport" content="initial-scale=1, width=device-width"/>
      </head>
      <body className={inter.className}>
      <Provider store={store}>
          <GlobalStateProvider>
              <ChakraProvider theme={theme}>
                  <MetaMaskProvider
                      debug={true}
                      sdkOptions={{
                          dappMetadata: {
                              name: process.env.WEBSITE_NAME,
                              // @ts-ignore
                              url: process.env.WEBSITE_URL,
                          },
                          infuraAPIKey: process.env.INFURA_API_KEY,
                      }}
                  >
                  <InitData></InitData>
                      <main>
                          <div className="mobile-display-none">
                              <Grid
                                  templateAreas={`"left center right"`}
                                  // gridTemplateColumns={'1fr 3fr 1fr'}
                                  gridTemplateColumns={'minmax(0, 1fr) minmax(0, 3fr) minmax(0, 1fr)'}
                                  gap={4}
                                  p={4}
                              >
                                  <Box gridArea="left" bg="gray.100" p={4} borderRadius="md">
                                      <Advertise></Advertise>
                                  </Box>
                                  <Box gridArea="center" bg="white" p={4} borderRadius="md" shadow="md" maxWidth="100%" width="100%">
                                      {children}
                                  </Box>
                                  <Box gridArea="right" bg="gray.100" p={4} borderRadius="md">
                                      <Menu></Menu>
                                  </Box>
                              </Grid>
                          </div>
                          <div className="desktop-display-none">
                              <Box gridArea="center" bg="white" p={4} borderRadius="md" shadow="md" >
                                  {children}
                              </Box>
                          </div>
                      </main>
                      <LargeWithNewsletter></LargeWithNewsletter>
                  <ScrollToTopButton />
                  </MetaMaskProvider>
              </ChakraProvider>
          </GlobalStateProvider>
      </Provider>
      </body>
      </html>
  );
}
