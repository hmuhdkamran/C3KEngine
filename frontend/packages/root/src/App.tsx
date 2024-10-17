import AppRoute from "@/router";
import { Provider } from "react-redux";
import { SystemStore } from "c3k-utilities";
import AxiosSetupComponent from "./components/utilities/axios-setup";

function App() {
  return (
    <>
      <Provider store={SystemStore}>
        <AxiosSetupComponent />
        <AppRoute />
      </Provider>
    </>
  );
}

export default App;
