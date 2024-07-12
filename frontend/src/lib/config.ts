interface Config {
    API_ENDPOINT: string;
    API_TIMEOUT: number;
    FRONTEND_DEV_MODE: boolean;
}

export const config: Config = {
    API_ENDPOINT: "http://localhost:8080/api/v1",
    API_TIMEOUT: 10,
    FRONTEND_DEV_MODE: true,
}