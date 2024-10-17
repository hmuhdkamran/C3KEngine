import { configureStore } from '@reduxjs/toolkit';
import dataReducer from './dataSlice';
import systemReducer from './systemSlice';

const store = configureStore({
    reducer: {
        data: dataReducer,
        system: systemReducer,
    },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

export default store;