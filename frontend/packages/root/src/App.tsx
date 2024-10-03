import AppRoute from "@/router";
import { useAxios } from "@/plugins/Api/axios";
import { SystemProvider } from "@/plugins/store/system";
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
