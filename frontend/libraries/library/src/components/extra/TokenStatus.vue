<script lang="ts" setup>
import type { IUser } from '@/models'
import { useSystemStore } from '@/store/system-store';
import { onMounted, ref, type Ref, watch } from 'vue';
import { useRouter } from 'vue-router';
import { useNotification } from '@/composables/useNotification';

interface Props {
    infoTimeout: number
    warnTimeout: number
    errorTimeout: number
    logout: boolean
    auth: any
}

const props = defineProps<Props>()

const { addNotification } = useNotification();

const router = useRouter()
const store = useSystemStore()

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
    tokenExpiresAt = new Date(tokenExpiresAt);
    
    if (tokenExpiresAt) {
        const ms = (tokenExpiresAt as Date).getTime() - new Date().getTime()
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
    addNotification(`Information: token expires at ${user.value.exp?.toLocaleString()}`, 'info', 'top-right', 'Info', 3000);
}

const warn = () => {
    addNotification(`Warning: token expires at ${user.value.exp?.toLocaleString()}`, 'warning', 'top-right', 'Warning', 3000);
}

const error = () => {
    if (logout.value) {
        props.auth.logout()
            .then((value: any) => store.updateUser(value))
            .then(() => addNotification(`Error: Token has been expired, Logging Out`, 'error', 'top-right', 'Error', 3000))
            .then(() => router.push({ name: 'login' }))
    }
    else { addNotification(`Error: Token has been expired, Logging Out`, 'error', 'top-right', 'Error', 3000) }
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
