import { Component } from '@angular/core';
import {PdfService} from "../../services/pdf.service";
import {MatSnackBar, MatSnackBarConfig} from "@angular/material/snack-bar";
import {Router} from "@angular/router";

@Component({
  selector: 'app-upload',
  templateUrl: './upload.component.html',
  styleUrls: ['./upload.component.css']
})
export class UploadComponent {

  files: File[] = [];

  progressSpinner = false;

  private snackBarConfig: MatSnackBarConfig = {
    duration: 2000
  };

  constructor(
    private pfdService: PdfService,
    private _snackBar: MatSnackBar,
    private router: Router,
  ) {
  }

  illegalFileType() {
    this._snackBar.open("Please only select pdf files", "Close", this.snackBarConfig);
  }

  addToFileArrayFromInput(event: Event) {
    const target = event.target as HTMLInputElement;
    const files = target.files as FileList;

    if (!files) {
      return;
    }

    this.addToFileArrayFromDrop(files);
  }


  addToFileArrayFromDrop(files: FileList) {

    for (let i = 0; i < files.length; i++) {
      let file = files.item(i);
      if (file && file.type === "application/pdf") {
        this.files.push(file);
      }
    }
  }

  uploadFiles() {

    if (this.files.length == 0) {
      this._snackBar.open("Please choose files", "Close", this.snackBarConfig);
      return;
    }

    this.progressSpinner = true;

    window.scrollTo(0, 0);

    const start = performance.now();

    this.pfdService.uploadFile(this.files).subscribe({
      next: data => {
        const end = performance.now();
        const diff = end - start;

        if (1000 - diff > 0) {
          setTimeout(_ => this.uploadSucceeded(), 1000 - diff);
        } else {
          this.uploadSucceeded();
        }
      },
      error: err => {
        const end = performance.now();
        const diff = end - start;

        if (1000 - diff > 0) {
          setTimeout(_ => this.uploadFailed(err), 1000 - diff);
        } else {
          this.uploadFailed(err);
        }
      }
    });
  }

  removeFromFileArray(index: number) {
    this.files.splice(index, 1);
  }

  clearSelectedFiles() {
    this.files = [];

    let input = document.getElementById('file-upload-in-box') as HTMLInputElement;
    input.value = '';
  }

  private uploadSucceeded() {
    this.progressSpinner = false;
    this._snackBar.open("Successfully uploaded files", "Close", this.snackBarConfig);
    this.clearSelectedFiles();
  }

  private uploadFailed(error: any) {
    this.progressSpinner = false;
    this._snackBar.open("There was an error uploading your files", "Close", this.snackBarConfig);
    this.clearSelectedFiles();
  }

}
