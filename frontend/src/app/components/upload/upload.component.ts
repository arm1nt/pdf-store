import { Component } from '@angular/core';
import {PdfService} from "../../services/pdf.service";
import {MatSnackBar} from "@angular/material/snack-bar";

@Component({
  selector: 'app-upload',
  templateUrl: './upload.component.html',
  styleUrls: ['./upload.component.css']
})
export class UploadComponent {

  files: File[] = [];

  constructor(
    private pfdService: PdfService,
    private _snackBar: MatSnackBar,
  ) {
  }

  uploadFile(event: Event) {
    const target = event.target as HTMLInputElement;
    const files = target.files as FileList;

    if(!files) {
      return;
    }

    this.uploadFile2(files);
  }

  uploadFile2(files: FileList) {
    for (let i = 0; i < files.length; i++) {
      let temp = files.item(i);
      if (temp) {
        this.files.push(temp);
      }
    }
  }

  uploadFiles() {
    if (this.files.length === 0) {
      this._snackBar.open("Please select files", "Close");
      return;
    }

    this.pfdService.uploadFile(this.files).subscribe({
      next: data => {
        this.files = [];
      },
      error: err => {
        //TODO: Display proper error msg
      }
    });
  }

  removeFromPossibleUploads(index: number) {
    this.files.splice(index, 1);
  }

}
