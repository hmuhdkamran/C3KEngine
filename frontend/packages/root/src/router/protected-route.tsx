import { routePath } from "@/pages";
import { Navigate } from "react-router-dom";
import { FC, ReactNode } from "react";
import { useSelector } from "react-redux";
import { RootState } from "c3k-utilities";

interface ProtectedRouteProps {
  element: ReactNode;
}

export const ProtectedRoute: FC<ProtectedRouteProps> = ({ element }) => {
  const { user } = useSelector((state: RootState) => state.system);
  return user.authenticated ? element : <Navigate to={routePath.LOGIN} />;
};
