import { routePath } from "@/pages";
import { Navigate } from "react-router-dom";
import { FC, ReactNode } from "react";
import { useSystemContext } from "c3k-utilities";

interface ProtectedRouteProps {
  element: ReactNode;
}

export const ProtectedRoute: FC<ProtectedRouteProps> = ({ element }) => {
  const { user } = useSystemContext();
  return user.authenticated ? element : <Navigate to={routePath.LOGIN} />;
};
