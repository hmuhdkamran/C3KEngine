import { TokenHelper } from "c3k-library";
import { reactive } from "vue";

export const store = reactive({
    user: TokenHelper.parseUserToken(TokenHelper.getAccessToken())
})