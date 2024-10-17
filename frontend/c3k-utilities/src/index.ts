import './styles/main.css';
import config from "./plugins/Api/config";
import store from "./plugins/store";

export { config as GlobalConfig };
export { store as SystemStore };

export * from "./components";

export * from "./plugins/Api/axios";

export * from "./plugins/helper/pick";
export * from "./plugins/helper/purry";
export * from "./plugins/helper/token-helper";

export * from "./plugins/service/payload-mapper";
export * from "./plugins/service/service";

export * from "./plugins/store/index";
export * from "./plugins/store/dataSlice";
export * from "./plugins/store/systemSlice";

export * from "./types/models";
export * from "./types/axios";
export * from "./types/configuration";