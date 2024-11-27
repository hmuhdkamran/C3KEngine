import { AuthenticationService } from "@/services/authentication-service";
import { TokenHelper } from "c3k-library";
import { reactive } from "vue";

const service: AuthenticationService = new AuthenticationService();

export const store = reactive({
    user: TokenHelper.parseUserToken(TokenHelper.getAccessToken()),
    userModules: await service.userProducts(),
    sideBarMneu: [],
})