import axios from "axios";
import { jwtDecode } from "jwt-decode";
import { decode } from "base-64";
import { Storage} from "./utils";
import { DecodedJWT } from "./types";

global.atob = decode;

export const BASE_URL = process.env.EXPO_PUBLIC_API_URL as string; // do pliku .env dodajcie EXPO_PUBLIC_API_URL z swoim adresem ip i poretm np. EXPO_PUBLIC_BASE_URL = "http://192.168.1.28:8000"

type AuthTokens = {
  access: string;
  refresh: string;
};

axios.defaults.baseURL = BASE_URL;
export const api = axios.create({
  baseURL: BASE_URL,
});

api.interceptors.request.use(async (config) => {
  let tokens = await getTokens();
  if (!tokens) {
    return config;
  }

  if (isExpired(tokens.access)) {
    tokens = await updateTokens(tokens.refresh);
  }

  config.headers["Authorization"] = `Bearer ${tokens.access}`;
  return config;
});
api.interceptors.response.use(
  (response) => response,
  async (error) => {
    if (error.response?.status === 401) {
      let tokens = await getTokens();

      try {
        if (tokens && !isExpired(tokens.refresh)) {
          tokens = await updateTokens(tokens.refresh);
          error.config.headers["Authorization"] = `Bearer ${tokens.access}`;
          return axios(error.config);
        }
      } catch (error) {
        console.error(error);
        await removeTokens();
      }
    }

    return Promise.reject(error);
  }
);

export async function removeTokens() {
  await Storage.remove("refreshToken");
  await Storage.remove("accessToken");
}

export async function resumeSession(): Promise<null>{
    const tokens = await getTokens();
    if (tokens) {
    //   return await whoAmI();
    return null;
    }
    return null;
}

function isExpired(token: string): boolean {
  try {
    const { exp }: DecodedJWT = jwtDecode(token);
    return new Date(exp).getTime() > Date.now();
  } catch (error) {
    console.error(error);
    return true;
  }
}

export async function updateTokens(token: string): Promise<AuthTokens> {
  const { data } = await axios.post("/api/token/refresh/", {
    refresh: token,
  });
  await setTokens(data);
  return data;
}

async function getTokens(): Promise<AuthTokens | null> {
    const refresh = await Storage.get("refreshToken");
    if (refresh === null) {
      return null;
    }
  
    return {
      refresh,
      access: (await Storage.get("accessToken")) ?? "",
    };
  }

  async function setTokens(newTokens: AuthTokens) {
    await Storage.set("refreshToken", newTokens.refresh);
    await Storage.set("accessToken", newTokens.access);
  }