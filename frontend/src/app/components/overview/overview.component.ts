import {Component, OnInit, ViewChild} from '@angular/core';
import {PdfOverviewDetails, PdfOverviewInfo} from "../../dtos/pdfOverview";
import {PdfService} from "../../services/pdf.service";
import {MatPaginator, PageEvent} from "@angular/material/paginator";

@Component({
  selector: 'app-overview',
  templateUrl: './overview.component.html',
  styleUrls: ['./overview.component.css']
})
export class OverviewComponent {

  pdfs: PdfOverviewDetails[] = [];
  page: number = 1;
  size: number = 24;
  total_number_of_pages: number | undefined = undefined;

  //@ViewChild(MatPaginator, undefined) paginator: MatPaginator;

  constructor(
    private pdfService: PdfService
  ) {
  }

  ngOnInit() {
    this.pdfService.getAllFilesPaged(this.page, this.size).subscribe(data => {
      this.pdfs = data.pdfs_previews;
      this.total_number_of_pages = data.count
      console.log("huhu: " + this.pdfs);
    });
  }

  getNextPage(e: PageEvent) {
    console.log(e);
    this.pdfService.getAllFilesPaged(e.pageIndex+1, e.pageSize).subscribe(data => {

      this.pdfs = data.pdfs_previews;
      this.total_number_of_pages = data.count
    });
  }




}
