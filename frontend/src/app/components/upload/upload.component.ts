import { Component } from '@angular/core';
import {PdfService} from "../../services/pdf.service";

@Component({
  selector: 'app-upload',
  templateUrl: './upload.component.html',
  styleUrls: ['./upload.component.css']
})
export class UploadComponent {
  constructor(
    private pfdService: PdfService
  ) {
  }

  uploadFile(event: Event) {
    // @ts-ignore
    this.pfdService.uploadFile(event.target.files).subscribe();
  }

}
