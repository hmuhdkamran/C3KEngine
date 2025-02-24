export interface IUser {
    UserId: string;
    Username: string;
    DisplayName: string;
    Language: string;
    Password: string;
    Salt: string;
    StatusId: string;
};

export interface IUsers {
    username: string;
    password: string;
    confirmPassword: string;
    displayName: string;
    language: string;
    role: string;
  }