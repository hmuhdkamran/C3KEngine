// vite.config.ts
import { fileURLToPath } from "node:url";
import { defineConfig } from "file:///mnt/Extended/Projects/Final/C3KEngine/frontend/node_modules/vite/dist/node/index.js";
import AutoImport from "file:///mnt/Extended/Projects/Final/C3KEngine/frontend/node_modules/unplugin-auto-import/dist/vite.js";
import Components from "file:///mnt/Extended/Projects/Final/C3KEngine/frontend/node_modules/unplugin-vue-components/dist/vite.js";
import Pages from "file:///mnt/Extended/Projects/Final/C3KEngine/frontend/node_modules/vite-plugin-pages/dist/index.js";
import Layouts from "file:///mnt/Extended/Projects/Final/C3KEngine/frontend/node_modules/vite-plugin-vue-layouts/dist/index.mjs";
import DefineOptions from "file:///mnt/Extended/Projects/Final/C3KEngine/frontend/node_modules/unplugin-vue-define-options/dist/vite.js";
import VueI18nPlugin from "file:///mnt/Extended/Projects/Final/C3KEngine/frontend/node_modules/@intlify/unplugin-vue-i18n/lib/vite.mjs";
import vue from "file:///mnt/Extended/Projects/Final/C3KEngine/frontend/node_modules/@vitejs/plugin-vue/dist/index.mjs";
import dotenv from "file:///mnt/Extended/Projects/Final/C3KEngine/frontend/node_modules/dotenv/lib/main.js";
var __vite_injected_original_import_meta_url = "file:///mnt/Extended/Projects/Final/C3KEngine/frontend/packages/root/vite.config.ts";
var envFile = ".env";
dotenv.config({ path: envFile });
var vite_config_default = defineConfig({
  plugins: [
    vue(),
    AutoImport({
      imports: ["vue", "vue-router", "@vueuse/core", "vue-i18n", "pinia"],
      vueTemplate: true
    }),
    Pages({
      dirs: ["./src/pages"]
    }),
    Components({
      dirs: ["src/components"],
      dts: true
    }),
    Layouts({
      layoutsDirs: "./src/layouts/"
    }),
    VueI18nPlugin({
      runtimeOnly: true,
      compositionOnly: true,
      include: [
        fileURLToPath(new URL("./src/plugins/i18n/locales/**", __vite_injected_original_import_meta_url))
      ]
    }),
    DefineOptions()
  ],
  server: {
    host: process.env.LOCAL_PATH,
    port: Number(process.env.LOCAL_PORT),
    open: false,
    watch: {
      usePolling: false,
      disableGlobbing: false
    }
  },
  define: { "process.env": {} },
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", __vite_injected_original_import_meta_url))
    }
  },
  build: {
    chunkSizeWarningLimit: 5e3,
    outDir: "../production/root",
    emptyOutDir: true,
    manifest: true,
    target: "esnext"
  }
});
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCIvbW50L0V4dGVuZGVkL1Byb2plY3RzL0ZpbmFsL0MzS0VuZ2luZS9mcm9udGVuZC9wYWNrYWdlcy9yb290XCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ZpbGVuYW1lID0gXCIvbW50L0V4dGVuZGVkL1Byb2plY3RzL0ZpbmFsL0MzS0VuZ2luZS9mcm9udGVuZC9wYWNrYWdlcy9yb290L3ZpdGUuY29uZmlnLnRzXCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ltcG9ydF9tZXRhX3VybCA9IFwiZmlsZTovLy9tbnQvRXh0ZW5kZWQvUHJvamVjdHMvRmluYWwvQzNLRW5naW5lL2Zyb250ZW5kL3BhY2thZ2VzL3Jvb3Qvdml0ZS5jb25maWcudHNcIjtpbXBvcnQgeyBmaWxlVVJMVG9QYXRoIH0gZnJvbSAnbm9kZTp1cmwnXG5pbXBvcnQgeyBkZWZpbmVDb25maWcgfSBmcm9tICd2aXRlJ1xuaW1wb3J0IEF1dG9JbXBvcnQgZnJvbSAndW5wbHVnaW4tYXV0by1pbXBvcnQvdml0ZSdcbmltcG9ydCBDb21wb25lbnRzIGZyb20gJ3VucGx1Z2luLXZ1ZS1jb21wb25lbnRzL3ZpdGUnXG5pbXBvcnQgUGFnZXMgZnJvbSAndml0ZS1wbHVnaW4tcGFnZXMnXG5pbXBvcnQgTGF5b3V0cyBmcm9tICd2aXRlLXBsdWdpbi12dWUtbGF5b3V0cydcbmltcG9ydCBEZWZpbmVPcHRpb25zIGZyb20gJ3VucGx1Z2luLXZ1ZS1kZWZpbmUtb3B0aW9ucy92aXRlJ1xuaW1wb3J0IFZ1ZUkxOG5QbHVnaW4gZnJvbSAnQGludGxpZnkvdW5wbHVnaW4tdnVlLWkxOG4vdml0ZSdcbmltcG9ydCB2dWUgZnJvbSAnQHZpdGVqcy9wbHVnaW4tdnVlJ1xuaW1wb3J0IGRvdGVudiBmcm9tICdkb3RlbnYnXG5cbmNvbnN0IGVudkZpbGUgPSAnLmVudidcblxuZG90ZW52LmNvbmZpZyh7IHBhdGg6IGVudkZpbGUgfSlcblxuLy8gaHR0cHM6Ly92aXRlanMuZGV2L2NvbmZpZy9cbmV4cG9ydCBkZWZhdWx0IGRlZmluZUNvbmZpZyh7XG4gIHBsdWdpbnM6IFtcbiAgICB2dWUoKSxcbiAgICBBdXRvSW1wb3J0KHtcbiAgICAgIGltcG9ydHM6IFsndnVlJywgJ3Z1ZS1yb3V0ZXInLCAnQHZ1ZXVzZS9jb3JlJywgJ3Z1ZS1pMThuJywgJ3BpbmlhJ10sXG4gICAgICB2dWVUZW1wbGF0ZTogdHJ1ZSxcbiAgICB9KSxcbiAgICBQYWdlcyh7XG4gICAgICBkaXJzOiBbJy4vc3JjL3BhZ2VzJ10sXG4gICAgfSksXG4gICAgQ29tcG9uZW50cyh7XG4gICAgICBkaXJzOiBbJ3NyYy9jb21wb25lbnRzJ10sXG4gICAgICBkdHM6IHRydWUsXG4gICAgfSksXG4gICAgTGF5b3V0cyh7XG4gICAgICBsYXlvdXRzRGlyczogJy4vc3JjL2xheW91dHMvJyxcbiAgICB9KSxcbiAgICBWdWVJMThuUGx1Z2luKHtcbiAgICAgIHJ1bnRpbWVPbmx5OiB0cnVlLFxuICAgICAgY29tcG9zaXRpb25Pbmx5OiB0cnVlLFxuICAgICAgaW5jbHVkZTogW1xuICAgICAgICBmaWxlVVJMVG9QYXRoKG5ldyBVUkwoJy4vc3JjL3BsdWdpbnMvaTE4bi9sb2NhbGVzLyoqJywgaW1wb3J0Lm1ldGEudXJsKSksXG4gICAgICBdLFxuICAgIH0pLFxuICAgIERlZmluZU9wdGlvbnMoKSxcbiAgXSxcbiAgc2VydmVyOiB7XG4gICAgaG9zdDogcHJvY2Vzcy5lbnYuTE9DQUxfUEFUSCxcbiAgICBwb3J0OiBOdW1iZXIocHJvY2Vzcy5lbnYuTE9DQUxfUE9SVCksXG4gICAgb3BlbjogZmFsc2UsXG4gICAgd2F0Y2g6IHtcbiAgICAgIHVzZVBvbGxpbmc6IGZhbHNlLFxuICAgICAgZGlzYWJsZUdsb2JiaW5nOiBmYWxzZSxcbiAgICB9LFxuICB9LFxuICBkZWZpbmU6IHsgJ3Byb2Nlc3MuZW52Jzoge30gfSxcbiAgcmVzb2x2ZToge1xuICAgIGFsaWFzOiB7XG4gICAgICAnQCc6IGZpbGVVUkxUb1BhdGgobmV3IFVSTCgnLi9zcmMnLCBpbXBvcnQubWV0YS51cmwpKSxcbiAgICB9LFxuICB9LFxuICBidWlsZDoge1xuICAgIGNodW5rU2l6ZVdhcm5pbmdMaW1pdDogNTAwMCxcbiAgICBvdXREaXI6ICcuLi9wcm9kdWN0aW9uL3Jvb3QnLFxuICAgIGVtcHR5T3V0RGlyOiB0cnVlLFxuICAgIG1hbmlmZXN0OiB0cnVlLFxuICAgIHRhcmdldDogJ2VzbmV4dCcsXG4gIH0sXG59KVxuIl0sCiAgIm1hcHBpbmdzIjogIjtBQUF5VyxTQUFTLHFCQUFxQjtBQUN2WSxTQUFTLG9CQUFvQjtBQUM3QixPQUFPLGdCQUFnQjtBQUN2QixPQUFPLGdCQUFnQjtBQUN2QixPQUFPLFdBQVc7QUFDbEIsT0FBTyxhQUFhO0FBQ3BCLE9BQU8sbUJBQW1CO0FBQzFCLE9BQU8sbUJBQW1CO0FBQzFCLE9BQU8sU0FBUztBQUNoQixPQUFPLFlBQVk7QUFUK00sSUFBTSwyQ0FBMkM7QUFXblIsSUFBTSxVQUFVO0FBRWhCLE9BQU8sT0FBTyxFQUFFLE1BQU0sUUFBUSxDQUFDO0FBRy9CLElBQU8sc0JBQVEsYUFBYTtBQUFBLEVBQzFCLFNBQVM7QUFBQSxJQUNQLElBQUk7QUFBQSxJQUNKLFdBQVc7QUFBQSxNQUNULFNBQVMsQ0FBQyxPQUFPLGNBQWMsZ0JBQWdCLFlBQVksT0FBTztBQUFBLE1BQ2xFLGFBQWE7QUFBQSxJQUNmLENBQUM7QUFBQSxJQUNELE1BQU07QUFBQSxNQUNKLE1BQU0sQ0FBQyxhQUFhO0FBQUEsSUFDdEIsQ0FBQztBQUFBLElBQ0QsV0FBVztBQUFBLE1BQ1QsTUFBTSxDQUFDLGdCQUFnQjtBQUFBLE1BQ3ZCLEtBQUs7QUFBQSxJQUNQLENBQUM7QUFBQSxJQUNELFFBQVE7QUFBQSxNQUNOLGFBQWE7QUFBQSxJQUNmLENBQUM7QUFBQSxJQUNELGNBQWM7QUFBQSxNQUNaLGFBQWE7QUFBQSxNQUNiLGlCQUFpQjtBQUFBLE1BQ2pCLFNBQVM7QUFBQSxRQUNQLGNBQWMsSUFBSSxJQUFJLGlDQUFpQyx3Q0FBZSxDQUFDO0FBQUEsTUFDekU7QUFBQSxJQUNGLENBQUM7QUFBQSxJQUNELGNBQWM7QUFBQSxFQUNoQjtBQUFBLEVBQ0EsUUFBUTtBQUFBLElBQ04sTUFBTSxRQUFRLElBQUk7QUFBQSxJQUNsQixNQUFNLE9BQU8sUUFBUSxJQUFJLFVBQVU7QUFBQSxJQUNuQyxNQUFNO0FBQUEsSUFDTixPQUFPO0FBQUEsTUFDTCxZQUFZO0FBQUEsTUFDWixpQkFBaUI7QUFBQSxJQUNuQjtBQUFBLEVBQ0Y7QUFBQSxFQUNBLFFBQVEsRUFBRSxlQUFlLENBQUMsRUFBRTtBQUFBLEVBQzVCLFNBQVM7QUFBQSxJQUNQLE9BQU87QUFBQSxNQUNMLEtBQUssY0FBYyxJQUFJLElBQUksU0FBUyx3Q0FBZSxDQUFDO0FBQUEsSUFDdEQ7QUFBQSxFQUNGO0FBQUEsRUFDQSxPQUFPO0FBQUEsSUFDTCx1QkFBdUI7QUFBQSxJQUN2QixRQUFRO0FBQUEsSUFDUixhQUFhO0FBQUEsSUFDYixVQUFVO0FBQUEsSUFDVixRQUFRO0FBQUEsRUFDVjtBQUNGLENBQUM7IiwKICAibmFtZXMiOiBbXQp9Cg==
