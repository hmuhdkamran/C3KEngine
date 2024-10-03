import {
  DashboardIndex,
  LandingPage,
  Login,
  RegisterForm,
  routePath,
  UserModule,
} from "@/pages";
import GernalLayout from "@/components/layout/gernal";
import AuthenticatedLayout from "@/components/layout/auth";

import { Route, Routes } from "react-router-dom";
import { ProtectedRoute } from "./protected-route";

const AppRoutes = () => {
  return (
    <>
      <Routes>
        <Route path={routePath.LOGIN} element={<Login />} />
        <Route path={routePath.REGISTER} element={<RegisterForm />} />
        <Route element={<GernalLayout />}>
          <Route path={routePath.LANDING} element={<LandingPage />} />
        </Route>
        <Route element={<AuthenticatedLayout />}>
          <Route
            path={routePath.DASHBOARD}
            element={<ProtectedRoute element={<DashboardIndex />} />}
          />

          <Route
            path={routePath.USER_MODULE}
            element={<ProtectedRoute element={<UserModule />} />}
          />
        </Route>
      </Routes>
    </>
  );
};

export default AppRoutes;
