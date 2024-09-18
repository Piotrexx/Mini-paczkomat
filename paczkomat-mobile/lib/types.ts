import { z } from "zod"

const email = z.string({ message: "Email jest wymagany" }).email("Niepoprawny adres email")
const password = z
  .string({ message: "Hasło jest wymagane" })
  .min(8, "Hasło musi mieć minimum 8 znaków")
  .max(30, "Hasło może mieć maksymalnie 30 znaków")


export const signUpSchema = z.object({
    email,
    password
})

export type User = {
    email: string,
    is_verified: boolean
}

export type AuthTokens = {
    access: string,
    refresh: string
}



export type DecodedJWT = {
    token_type: string;
    exp: number;
    iat: number;
    jti: string;
    user_id: number;
  };
  

export type SignUpSchema = z.infer<typeof signUpSchema>;