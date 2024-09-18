import React, { useState } from 'react';
import { View, Text, TextInput, Button, TouchableOpacity } from 'react-native';
import axios from 'axios';
import { useAuth } from '@/components/AuthProvider';
import { router } from 'expo-router';

const LoginScreen = () => {
  const { login } = useAuth();
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  const handleLogin = async () => {
    try {
      const response = await axios.post('https://your-api-url.com/login', { email, password });
      await login(response.data.token);
    } catch (error) {
      console.error('Login failed:', error);
    }
  };

  return (
    <View>
      <TextInput placeholder="Email" value={email} onChangeText={setEmail} />
      <TextInput placeholder="Password" value={password} onChangeText={setPassword} secureTextEntry />
      <Button title="Login" onPress={handleLogin} />
      <TouchableOpacity onPress={() => router.push("/auth/register")}>
        <Text>Don't have an account? Register here!</Text>
      </TouchableOpacity>
    </View>
  );
};

export default LoginScreen;