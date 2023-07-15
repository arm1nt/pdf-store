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

    if (id === null) {
      this.router.navigate(["/overview"]);
    }

    // @ts-ignore
    this.pdfService.getById(id).subscribe({
      next: value => {
        this.pdfDetails = value;
        console.log(value);
      },
      error: err => {
        this._snackBar.open("No pdf with given id found.", "Close");

      }
      }
    )


    //console.log(this.route.snapshot.paramMap.get('id'));
    /*this.route.params.subscribe(data => {
      console.log(data);
    });*/

  }

}
