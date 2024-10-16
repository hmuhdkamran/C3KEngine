import AppRoute from "@/router";
import { SystemProvider, useAxios } from "c3k-utilities";

function App() {
  useAxios();

  return (
    <>
      <SystemProvider>
        <AppRoute />
      </SystemProvider>
    </>
  );
}

export default App;
