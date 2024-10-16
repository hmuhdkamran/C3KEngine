import AppRoute from "@/router";
import { SystemProvider } from "c3k-utilities";
import AxiosSetupComponent from "./components/utilities/axios-setup";

function App() {
  return (
    <>
      <SystemProvider>
        <AxiosSetupComponent />
        <AppRoute />
      </SystemProvider>
    </>
  );
}

export default App;
