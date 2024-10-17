export interface IUsers {
    UserId: string
    Username: string
    DisplayName: string
    Language: string
    Password: string
    Salt: string
    Status: boolean
  }
  
  export interface IUser {
    username: string;
    password: string;
    confirmPassword: string;
    displayName: string;
    language: string;
    role: string;
  }
  