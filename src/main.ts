import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog"
import { basename, dirname } from '@tauri-apps/api/path';
import { exit } from "@tauri-apps/api/process";

let msgFileInput: HTMLElement;
let msgDirInput: HTMLElement;
let msgDirOutput: HTMLElement;
let menuSelectWorksheet: HTMLElement;

let fileInput: string|null = null;
let fileOutput: string = "";
let dirInput: string = ".";
let dirOutput: string = ".";
let workSheet: string = "";
let workSheets: string[] = [];


async function openDialogFileInput(): Promise<void> {
    const fileInputTmp = await open({
        filters: [{
            name: "spreadsheet",
            extensions: ["xlsx"]
        }]}
    ) as string;
    if (fileInputTmp != null) {
        fileInput = fileInputTmp;
        dirInput = await dirname(fileInput);
        dirOutput = await dirname(fileInput);
        fileOutput = "shuffled-" + await basename(fileInput);

        // display file/folder path
        msgFileInput.textContent = fileInput;
        msgDirInput.textContent = dirInput;
        msgDirOutput.textContent = dirOutput;

        // get worksheet name (Rust)
        workSheets = await invoke("get_worksheet", {
            fileInput: fileInput
        });

        // delete present menu items
        while (menuSelectWorksheet.firstChild) {
          menuSelectWorksheet.removeChild(menuSelectWorksheet.firstChild);
        }
        // set pulldown menu
        for(let i = 0; i < workSheets.length; i++){
            let op = document.createElement("option");
            op.value = i.toString();
            op.text = workSheets[i];
            menuSelectWorksheet.appendChild(op)!;
        }
        workSheet = workSheets[0];
    }
}


async function selectWorksheet(): Promise<void> {
    workSheet = workSheets[menuSelectWorksheet.value];
}


async function openDialogDirInput(): Promise<void> {
    dirInput = await open({
        directory: true,
        defaultPath: dirInput
    }) as string;
    if (dirInput != null) {
        msgDirInput.textContent = dirInput;
    }
}

async function openDialogDirOutput(): Promise<void> {
    dirOutput = await open({
        directory: true,
        defaultPath: dirOutput
    }) as string;
    if (dirOutput != null) {
        msgDirOutput.textContent = dirOutput;
    }
}

async function doShuffle() {
    if (fileInput != null) {
        await invoke("do_shuffle", {
            fileInput: fileInput,
            dirInput: dirInput,
            fileOutput: fileOutput,
            dirOutput: dirOutput,
            workSheet: workSheet
        });
    }
}

async function quit() {
    await exit(0);
}


window.addEventListener("DOMContentLoaded", () => {
  msgFileInput = document.querySelector("#msg_file_input")!;
  msgDirInput = document.querySelector("#msg_dir_input")!;
  msgDirOutput = document.querySelector("#msg_dir_output")!;
  menuSelectWorksheet = document.querySelector("#btn_select_worksheet")!;

  document
    .querySelector("#btn_select_file_input")
    ?.addEventListener("click", () => openDialogFileInput());
  document
    .querySelector("#btn_select_worksheet")
    ?.addEventListener("change", () => selectWorksheet());
  document
    .querySelector("#btn_select_dir_input")
    ?.addEventListener("click", () => openDialogDirInput());
  document
    .querySelector("#btn_select_dir_output")
    ?.addEventListener("click", () => openDialogDirOutput());
  document
    .querySelector("#btn_do")
    ?.addEventListener("click", () => doShuffle());
  document
    .querySelector("#btn_quit")
    ?.addEventListener("click", () => quit());
});
