<script lang="ts" setup>
import type { IUser } from '@/plugins/models'
import { useSystemStore } from '@/plugins/store/system-store';
import { AuthenticationService } from '@/service/auth/authentication-service';
import { onMounted, ref, Ref, watch } from 'vue';
import { useRouter } from 'vue-router';

interface Props {
    infoTimeout: number
    warnTimeout: number
    errorTimeout: number
    logout: boolean
}

const props = defineProps<Props>()

const router = useRouter()
const store = useSystemStore()

const auth: AuthenticationService = new AuthenticationService()
const user: Ref<IUser> = ref(store.user)

const infoTimeout: Ref<number> = ref(props.infoTimeout)
const warnTimeout: Ref<number> = ref(props.warnTimeout)
const errorTimeout: Ref<number> = ref(props.errorTimeout)
const logout: Ref<boolean> = ref(props.logout)

const tokenExpiryChanged = (tokenExpiresAt: Date) => {
    clearHandlers()

    if (tokenExpiresAt) {
        setHandlers(tokenExpiresAt);
    }
}

const clearHandlers = () => {
    if (infoTimeout)
        window.clearTimeout(infoTimeout.value)

    if (warnTimeout)
        window.clearTimeout(warnTimeout.value)

    if (errorTimeout)
        window.clearTimeout(errorTimeout.value)
}

const setHandlers = (tokenExpiresAt: Date) => {
    if (tokenExpiresAt) {
        const ms = tokenExpiresAt.getTime() - new Date().getTime()
        const seconds = ms / 1000

        if (props.infoTimeout && seconds > props.infoTimeout)
            infoTimeout.value = window.setTimeout(() => info(), ms - props.infoTimeout * 1000)

        if (props.warnTimeout && seconds > props.warnTimeout)
            warnTimeout.value = window.setTimeout(() => warn(), ms - props.warnTimeout * 1000)

        if (props.errorTimeout)
            errorTimeout.value = window.setTimeout(() => error(), tokenExpiresAt.getTime() - new Date().getTime())
    }
}

const info = () => {
    console.log(`token expires at ${user.value.exp?.toLocaleString()}`);
}

const warn = () => {
    console.log(`token expires at ${user.value.exp?.toLocaleString()}`);
}

const error = () => {
    if (logout.value) {
        auth.logout()
            .then((value: any) => store.updateUser(value))
            .then(() => console.log("Token  expired, logged out"))
            .then(() => router.push({ name: 'login' }))
    }
    else { console.log("Token  expired, logged out"); }
}

onMounted(() => {
    if (user.value == null)
        user.value = store.user

    if (user.value.exp) {
        setHandlers(user.value.exp)
        watch(() => user.value.exp, tokenExpiryChanged)
    }
})
</script>

<template>
    <div />
</template>
