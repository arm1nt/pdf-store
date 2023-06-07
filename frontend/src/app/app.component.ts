import { Component } from '@angular/core';
import {PdfService} from "./services/pdf.service";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {
  title = 'frontend';

  constructor(
    private pfdService: PdfService
  ) {
  }

  uploadFile(event: Event) {


    // @ts-ignore
    this.pfdService.uploadFile(event.target.files).subscribe();
  }

}
