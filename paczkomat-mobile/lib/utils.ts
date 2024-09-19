import { Platform } from "react-native";
import * as SecureStore from "expo-secure-store";

export const Storage = {
    get: async (key: string) => {
      if (Platform.OS === "web") return localStorage.getItem(key);
      return await SecureStore.getItemAsync(key);
    },
    set: async (key: string, item: string) => {
      if (Platform.OS === "web") localStorage.setItem(key, item);
      await SecureStore.setItemAsync(key, item);
    },
    remove: async (key: string) => {
      if (Platform.OS === "web") localStorage.removeItem(key);
      await SecureStore.deleteItemAsync(key);
    },
  };


  export function capitalizeFirstLetter(string: string | undefined) {
    if (!string) return
    return string.charAt(0).toUpperCase() + string.slice(1);
  }