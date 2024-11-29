<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';

interface Props {
    path: string;
    data: any;
}

interface Emit {
  (e: "close"): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emit>();

Stimulsoft.Base.StiLicense.key = '';
var options = new Stimulsoft.Viewer.StiViewerOptions();
options.height = "100%";
options.appearance.scrollbarsMode = true;
options.toolbar.showDesignButton = false;
options.toolbar.printDestination = Stimulsoft.Viewer.StiPrintDestination.Direct;
options.appearance.htmlRenderMode = Stimulsoft.Report.Export.StiHtmlExportMode.Table;

var viewer = new Stimulsoft.Viewer.StiViewer(options, "StiViewer", false);
var report = new Stimulsoft.Report.StiReport();
var dataSet = new Stimulsoft.System.Data.DataSet("root");

dataSet.readJson(JSON.stringify(this.data));

report.loadFile(this.path);
report.dictionary.databases.clear();

report.regData("root", "root", dataSet);
report.dictionary.synchronize();

viewer.report = report;
viewer.renderHtml("viewerContent");
</script>

<template>
    <div class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50 z-50">
        <div class="bg-white rounded-md shadow-lg w-96 overflow-hidden">
            <!-- Title with Primary Background -->
            <h3 class="text-gray-700 font-semibold text-md p-3">
                Print Preview
            </h3>
            <!-- Message -->
            <div class="p-6">
                <div id="viewerContent" ref="viewerContent"></div>
            </div>
            <!-- Action Buttons -->
            <div class="bg-gray-100 p-3 flex justify-end space-x-2">
                <button class="px-3 py-1 text-sm btn-secondary" @click="emit('close')">Close</button>
            </div>
        </div>
    </div>
</template>