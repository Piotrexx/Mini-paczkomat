import { useEffect } from "react";
import { Pressable } from "react-native";
import { Stack, router } from "expo-router";
import { StatusBar } from "expo-status-bar";
import { GestureHandlerRootView } from "react-native-gesture-handler";

export default function AuthLayout() {
  
  return (
    <>
    <StatusBar style="light" />
      <Stack>
        <Stack.Screen
          name="register"
          options={{
            headerShadowVisible: false,
            title: "Rejestracja",
            headerBackVisible: false,}}
            />
    </Stack>
    </>
   
  );
}