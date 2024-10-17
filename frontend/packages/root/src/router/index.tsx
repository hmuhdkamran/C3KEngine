import {
  DashboardIndex,
  LandingPage,
  Login,
  RegisterForm,
  routePath,
  UserModule,
} from "@/pages";

import { DataProvider, PageProvider } from "c3k-utilities";
import Layout from "@/components/layout/layout";

import { Route, Routes } from "react-router-dom";
import { ProtectedRoute } from "./protected-route";

const AppRoutes = () => {
  return (
    <Routes>
      <Route path={routePath.LOGIN} element={<Login />} />
      <Route path={routePath.REGISTER} element={<RegisterForm />} />
      <Route element={<Layout isAuthenticated={false} />}>
        <Route path={routePath.LANDING} element={<LandingPage />} />
      </Route>
      <Route
        element={
          <DataProvider>
            <PageProvider>
              <Layout isAuthenticated={true} />
            </PageProvider>
          </DataProvider>
        }
      >
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
  );
};

export default AppRoutes;
