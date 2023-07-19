import {Component, OnInit} from '@angular/core';
import {ActivatedRoute, Router} from "@angular/router";
import {PdfService} from "../../services/pdf.service";
import {PdfDetails} from "../../dtos/pdfDetails";
import {MatSnackBar} from "@angular/material/snack-bar";

@Component({
  selector: 'app-pdf-detail-view',
  templateUrl: './pdf-detail-view.component.html',
  styleUrls: ['./pdf-detail-view.component.css']
})
export class PdfDetailViewComponent implements OnInit {

  pdfId: string | null | undefined;
  pdfDetails: PdfDetails | undefined;
  testChips = ["Test", "Test","Test","Test","Test","Test","Test","Test","Test","Test","Test","Test","Test","Test","Test",
    "Test","Test","Test","Test","Test","Test","Test","Test","Test","Test","Test","Test","Test","Test","Test","Test",
    "Test","Test","Test","Test","Test","Test","Test","Test","Test","Test","Test","Test","Test","Test","Test","Test"];

  constructor(
    private pdfService: PdfService,
    private router: Router,
    private route: ActivatedRoute,
    private _snackBar: MatSnackBar
  ) {
  }

  ngOnInit() {

    let id = this.route.snapshot.paramMap.get('id');
    this.pdfId = id;

    if (id === null) {
      this.router.navigate(["/overview"]);
    }
    // @ts-ignore
    this.pdfService.getMetadataById(id).subscribe({
      next: value => {
        this.pdfDetails = value;
      },
      error: err => {
        this._snackBar.open("No pdf with given id found.", "Close");
        this.router.navigate(["/overview"]);
      }
      }
    )
  }

  bytesToBase64(bytes:any ) {

    const base64abc = [
      "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M",
      "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
      "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m",
      "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
      "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "+", "/"
    ];

    let result = '', i, l = bytes.length;
    for (i = 2; i < l; i += 3) {
      result += base64abc[bytes[i - 2] >> 2];
      result += base64abc[((bytes[i - 2] & 0x03) << 4) | (bytes[i - 1] >> 4)];
      result += base64abc[((bytes[i - 1] & 0x0F) << 2) | (bytes[i] >> 6)];
      result += base64abc[bytes[i] & 0x3F];
    }
    if (i === l + 1) { // 1 octet yet to write
      result += base64abc[bytes[i - 2] >> 2];
      result += base64abc[(bytes[i - 2] & 0x03) << 4];
      result += "==";
    }
    if (i === l) { // 2 octets yet to write
      result += base64abc[bytes[i - 2] >> 2];
      result += base64abc[((bytes[i - 2] & 0x03) << 4) | (bytes[i - 1] >> 4)];
      result += base64abc[(bytes[i - 1] & 0x0F) << 2];
      result += "=";
    }
    return result;
  }

  toBlob(pdfArray: string) {
    const byteCharacters = atob(pdfArray);
    const byteNumbers = new Array(byteCharacters.length);
    for (let i = 0; i < byteCharacters.length; i++) {
      byteNumbers[i] = byteCharacters.charCodeAt(i);
    }
    const byteArray = new Uint8Array(byteNumbers);
    return new Blob([byteArray], {type: 'application/pdf'});

  }

  getPdf(download: boolean) {
    if (!this.pdfId) {
      //TODO: error message
      return;
    }

    this.pdfService.getById(this.pdfId).subscribe({
      next: data => {
        if (download) {
          this.downloadPdf(data.pdf);
        } else {
          this.viewPdf(data.pdf);
        }
      },
      error: err => { /*TODO: Error msg*/}
    });
  }

  downloadPdf(pdfArray: string) {
    const file = this.toBlob(pdfArray);
    const url = URL.createObjectURL(file);
    const link = document.createElement('a');
    link.href = url;
    link.download = `${this.pdfDetails?.file_name}`;
    link.click();

    URL.revokeObjectURL(url);
    link.remove();
  }

  viewPdf(pdfArray: string) {
    const file = this.toBlob(pdfArray);

    const fileUrl = URL.createObjectURL(file);
    window.open(fileUrl)

    URL.revokeObjectURL(fileUrl);
  }

}
