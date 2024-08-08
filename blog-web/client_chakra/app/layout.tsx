"use client";

import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import LargeWithNewsletter from "./common/footer";
import {GlobalStateProvider} from "@/context/GlobalState";
import {Box, ChakraProvider, Grid} from "@chakra-ui/react";
import { extendTheme } from '@chakra-ui/react';
const inter = Inter({ subsets: ["latin"] });
import React from "react";
import Advertise from "./advertise/page";
import Menu from  "./menu/page";
import ScrollToTopButton from "@/app/common/ScrollToTopButton";

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
      <GlobalStateProvider>
        <ChakraProvider theme={theme}>
          <main>
            <div className="mobile-display-none">
              <Grid
                  templateAreas={`"left center right"`}
                  gridTemplateColumns={'1fr 3fr 1fr'}
                  gap={4}
                  p={4}
              >
                <Box gridArea="left" bg="gray.100" p={4} borderRadius="md">
                  <Advertise></Advertise>
                </Box>
                <Box gridArea="center" bg="white" p={4} borderRadius="md" shadow="md">
                  {children}
                </Box>
                <Box gridArea="right" bg="gray.100" p={4} borderRadius="md">
                  <Menu></Menu>
                </Box>
              </Grid>
            </div>
            <div className="desktop-display-none">
              <Box gridArea="center" bg="white" p={4} borderRadius="md" shadow="md">
                {children}
              </Box>
            </div>
          </main>
          <LargeWithNewsletter></LargeWithNewsletter>
          <ScrollToTopButton />
        </ChakraProvider>
      </GlobalStateProvider>
      </body>
      </html>
  );
}
