<script setup lang="ts">
import { defineProps, withDefaults, computed } from 'vue';

interface CornerConfig {
    tl?: string;
    tr?: string;
    bl?: string;
    br?: string;
    t?: string;
    b?: string;
    l?: string;
    r?: string;
}

interface BorderConfig {
    all?: string;
    top?: string;
    right?: string;
    bottom?: string;
    left?: string;
    color?: string;
}

interface CardProps {
    /**
     * Corner configuration:
     * - As a string: "none", "sm", "md", "lg" (applied to all corners)
     * - As an object: { tl?: string, tr?: string, bl?: string, br?: string }
     */
    corner?: string | CornerConfig;
    /**
     * Border configuration:
     * - As a string: any Tailwind border classes (or a numeric value for width)
     * - As an object: { 
     *     all?: string, top?: string, right?: string, bottom?: string, left?: string,
     *     color?: string // e.g., "border-blue-500"
     *   }
     */
    border?: string | BorderConfig;
    /**
     * Drop shadow class (e.g., "shadow-sm"); defaults to "none"
     */
    shadow?: string;
    /**
     * Hover drop shadow:
     * - As a string (e.g., "shadow-lg") or boolean true (which applies a default hover shadow)
     * Defaults to "none"
     */
    hoverShadow?: string | boolean;
}

const props = withDefaults(defineProps<CardProps>(), {
    corner: "none",
    border: "none",
    shadow: "none",
    hoverShadow: "none"
});

const cast = (value: any): string => {
    if (typeof value === 'string') {
        return value;
    } else if (!isNaN(Number(value))) {
        return `[${value}px]`;
    } else {
        return '';
    }
}

// Compute corner classes
const cornerClass = computed(() => {
    if (typeof props.corner === 'string') {
        return props.corner === 'none' ? '' : `rounded-${cast(props.corner)}`;
    } else if (typeof props.corner === 'object') {
        let classes: string[] = [];
        if (props.corner.tl) classes.push(`rounded-tl-${cast(props.corner.tl)}`);
        if (props.corner.tr) classes.push(`rounded-tr-${cast(props.corner.tr)}`);
        if (props.corner.bl) classes.push(`rounded-bl-${cast(props.corner.bl)}`);
        if (props.corner.br) classes.push(`rounded-br-${cast(props.corner.br)}`);
        if (props.corner.t) classes.push(`rounded-t-${cast(props.corner.tl)}`);
        if (props.corner.b) classes.push(`rounded-b-${cast(props.corner.tr)}`);
        if (props.corner.l) classes.push(`rounded-l-${cast(props.corner.bl)}`);
        if (props.corner.r) classes.push(`rounded-r-${cast(props.corner.br)}`);
        return classes.join(' ');
    }
    return '';
});

// Compute border classes
const borderClass = computed(() => {
    if (typeof props.border === 'string') {
        return props.border === 'none' ? '' : props.border;
    } else if (typeof props.border === 'object') {
        let classes: string[] = [];
        if (props.border.all) classes.push(`border-${cast(props.border.all)}`);
        if (props.border.top) classes.push(`border-t-${cast(props.border.top)}`);
        if (props.border.right) classes.push(`border-r-${cast(props.border.right)}`);
        if (props.border.bottom) classes.push(`border-b-${cast(props.border.bottom)}`);
        if (props.border.left) classes.push(`border-l-${cast(props.border.left)}`);
        if (props.border.color) classes.push(`border-${props.border.color}`);
        return classes.join(' ');
    }
    return '';
});

// Compute shadow classes
const shadowClass = computed(() => {
    return props.shadow === 'none' ? '' : props.shadow;
});
const hoverShadowClass = computed(() => {
    if (props.hoverShadow === 'none') {
        return '';
    }
    if (typeof props.hoverShadow === 'boolean' && props.hoverShadow === true) {
        return 'hover:shadow-md';
    }
    return `hover:${props.hoverShadow}`;
});

// Combine classes for the card container
const cardClasses = computed(() => {
    return `relative ${cornerClass.value} ${borderClass.value} ${shadowClass.value} ${hoverShadowClass.value}`;
});
</script>

<template>
    <div :class="[cardClasses, $attrs.class]">
        <template v-if="$slots.header">
            <div class="mb-0 border-b border-gray-300 pt-3 pb-2 px-1">
                <span class="text-sm text-slate-600 font-medium">
                    <slot name="header" />
                </span>
            </div>
        </template>
        <slot />
        <template v-if="$slots.footer">
            <div class="border-t border-gray-300 pb-3 pt-2 px-1">
                <span class="text-sm text-slate-600 font-medium">
                    <slot name="footer" />
                </span>
            </div>
        </template>        
    </div>
</template>

<style scoped></style>