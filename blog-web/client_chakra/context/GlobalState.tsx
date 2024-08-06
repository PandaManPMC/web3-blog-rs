// context/GlobalState.js
import React, { createContext, useContext, useState } from 'react';

// @ts-ignore
const GlobalStateContext = createContext();

// @ts-ignore
export const GlobalStateProvider = ({ children }) => {
    const [state, setState] = useState({
        user: null,
        theme: 'light',
    });

    return (
        <GlobalStateContext.Provider value={[state, setState]}>
            {children}
        </GlobalStateContext.Provider>
    );
};

export const useGlobalState = () => useContext(GlobalStateContext);
