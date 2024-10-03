import { routePath } from "@/pages";
import { useSystemContext } from "@/plugins/store";
import { Navigate } from "react-router-dom";
import { FC, ReactNode } from "react";

interface ProtectedRouteProps {
  element: ReactNode;
}

export const ProtectedRoute: FC<ProtectedRouteProps> = ({ element }) => {
  const { user } = useSystemContext();
  return user.authenticated ? element : <Navigate to={routePath.LOGIN} />;
};
