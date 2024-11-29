<script setup lang="ts">
import { onMounted, ref, defineProps, defineEmits } from 'vue';

declare var Stimulsoft: any;

interface Props {
    path: string;
    data: any;
}

interface Emit {
    (e: 'close'): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emit>();

// Reference for the viewer container
const viewerContainer = ref<HTMLDivElement | null>(null);

// Stimulsoft Viewer Initialization
class ReportViewer {
    private viewer: any;
    private report: any;

    constructor() {
        this.initLicense();
        this.viewer = new Stimulsoft.Viewer.StiViewer(this.viewerOptions(), "StiViewer", false);
        this.report = new Stimulsoft.Report.StiReport();
    }

    // Set the license key
    private initLicense() {
        Stimulsoft.Base.StiLicense.key = '6vJhGtLLLz2GNviWmUTrhSqnOItdDwjBylQzQcAOiHkgpgFGkUl79uxVs8X+uspx6K+tqdtOB5G1S6PFPRrlVNvMUiSiNYl724EZbrUAWwAYHlGLRbvxMviMExTh2l9xZJ2xc4K1z3ZVudRpQpuDdFq+fe0wKXSKlB6okl0hUd2ikQHfyzsAN8fJltqvGRa5LI8BFkA/f7tffwK6jzW5xYYhHxQpU3hy4fmKo/BSg6yKAoUq3yMZTG6tWeKnWcI6ftCDxEHd30EjMISNn1LCdLN0/4YmedTjM7x+0dMiI2Qif/yI+y8gmdbostOE8S2ZjrpKsgxVv2AAZPdzHEkzYSzx81RHDzZBhKRZc5mwWAmXsWBFRQol9PdSQ8BZYLqvJ4Jzrcrext+t1ZD7HE1RZPLPAqErO9eo+7Zn9Cvu5O73+b9dxhE2sRyAv9Tl1lV2WqMezWRsO55Q3LntawkPq0HvBkd9f8uVuq9zk7VKegetCDLb0wszBAs1mjWzN+ACVHiPVKIk94/QlCkj31dWCg8YTrT5btsKcLibxog7pv1+2e4yocZKWsposmcJbgG0';
    }

    // Viewer configuration
    private viewerOptions() {
        const options = new Stimulsoft.Viewer.StiViewerOptions();
        options.height = '100%';
        options.appearance.scrollbarsMode = true;
        options.toolbar.showDesignButton = false;
        options.toolbar.printDestination = Stimulsoft.Viewer.StiPrintDestination.Direct;
        options.appearance.htmlRenderMode = Stimulsoft.Report.Export.StiHtmlExportMode.Table;
        return options;
    }

    // Load the report with data
    public loadReport(path: string, data: any) {
        const dataSet = new Stimulsoft.System.Data.DataSet('root');
        dataSet.readJson(JSON.stringify(data));

        this.report.loadFile(path);
        this.report.dictionary.databases.clear();
        this.report.regData('root', 'root', dataSet);
        this.report.dictionary.synchronize();

        this.viewer.report = this.report;
    }

    // Render the viewer in the container
    public render(container: HTMLElement | null) {
        if (!container) return;
        this.viewer.renderHtml(container.id);
    }
}

// On component mount
onMounted(() => {
    const viewer = new ReportViewer();
    viewer.loadReport(props.path, props.data);
    viewer.render(viewerContainer.value);
});
</script>

<template>
    <div class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50 z-50">
        <div class="bg-white rounded shadow-lg w-full max-w-5xl overflow-hidden">
            <!-- Title -->
            <h3 class="text-gray-700 font-semibold text-md p-4 border-b">
                Print Preview
            </h3>
            <!-- Viewer Content -->
            <div class="p-6" ref="viewerContainer" id="viewerContainer"></div>
            <!-- Action Buttons -->
            <div class="bg-gray-100 p-3 flex justify-end space-x-2 border-t">
                <button class="px-4 py-2 btn-secondary text-sm" @click="emit('close')">Close</button>
            </div>
        </div>
    </div>
</template>

<style scoped>
/* Style overrides for buttons and modal */
.btn-secondary {
    background-color: #e2e8f0;
    color: #4a5568;
    border: 1px solid #cbd5e0;
    border-radius: 0.375rem;
    transition: background-color 0.2s;
}

.btn-secondary:hover {
    background-color: #cbd5e0;
}
</style>
