import { AxiosError, InternalAxiosRequestConfig, AxiosResponse } from 'axios';
import Axios from 'axios';
import { useNavigate } from 'react-router-dom';
import { useEffect } from 'react';
import { TokenHelper } from '../helper/token-helper';

let initialized = false;

function handle(status: number, exclude: number[]) {
    return exclude.length === 0 || exclude.find(o => o === status) === undefined;
}

export function useAxios() {
    const navigate = useNavigate();

    useEffect(() => {
        if (!initialized) {
            // Axios request interceptor
            Axios.interceptors.request.use((config: InternalAxiosRequestConfig) => {
                // Example: Add bearer token to headers if needed
                if (!config.headers?.Authorization) {
                  const bearerToken = TokenHelper.getBearerToken();
                  if (bearerToken.Authorization) {
                    config.headers.Authorization = bearerToken.Authorization;
                  }
                }

                // Ensure headers object exists before modifying it
                config.headers = config.headers || {};

                // Set max redirects if not explicitly set
                if (!config.maxRedirects || config.maxRedirects === 5) {
                    config.maxRedirects = 0;
                }

                return config;
            });

            // Axios response interceptor
            Axios.interceptors.response.use(
                response => response, // Pass through successful responses
                (error: AxiosError) => {
                    const response: AxiosResponse | undefined = error.response;
                    const exclude: number[] = [];

                    // Handle 401 Unauthorized errors
                    if (response?.status === 401 && handle(response.status, exclude)) {
                        const location: string | undefined =
                            response.headers?.location || response.headers?.Location;

                        if (location) {
                            const redirectTo = `/${location}`;
                            setTimeout(() => navigate(redirectTo), 200);
                        }
                    }

                    // Handle 403 Forbidden errors
                    if (response?.status === 403 && handle(response.status, exclude)) {
                        setTimeout(() => navigate('/forbidden'), 200);
                    }

                    return Promise.reject(error); // Reject the promise with the error
                }
            );

            initialized = true;
        }
    }, [navigate]); // Ensure the interceptor is set only once
}
