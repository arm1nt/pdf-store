import {Component, OnInit, ViewChild} from '@angular/core';
import {PdfOverviewDetails, PdfOverviewInfo} from "../../dtos/pdfOverview";
import {PdfService} from "../../services/pdf.service";
import {MatPaginator, PageEvent} from "@angular/material/paginator";
import {PdfSearch} from "../../dtos/pdfSearch";

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
  expand_detail_view = false;

  search_title:  string | undefined = undefined;
  search_author: string | undefined = undefined;
  search_tags: string | undefined = undefined;

  constructor(
    private pdfService: PdfService
  ) {
  }

  ngOnInit() {
    this.pdfService.getAllFilesPaged(this.page, this.size).subscribe(data => {
      this.pdfs = data.pdfs_previews;
      this.total_number_of_pages = data.count
    });
  }

  getNextPage(e: PageEvent) {
    this.pdfService.getAllFilesPaged(e.pageIndex+1, e.pageSize).subscribe(data => {

      this.pdfs = data.pdfs_previews;
      this.total_number_of_pages = data.count
    });
  }

  showDetailView() {
    this.expand_detail_view = !this.expand_detail_view;
  }

  clear() {
    this.search_title = undefined;
    this.search_author = undefined;
    this.search_tags = undefined;
  }

  search() {

    if (this.search_title?.trim().length == 0) {
      this.search_title = undefined;
    }

    if (this.search_author?.trim().length == 0) {
      this.search_author = undefined;
    }

    if (this.search_tags?.trim().length == 0) {
      this.search_tags = undefined;
    }

    if ( this.search_tags == undefined &&
      this.search_title == undefined &&
      this.search_author == undefined) {
      this.page = 1;
      this.pdfService.getAllFilesPaged(this.page, this.size).subscribe(data => {
        this.pdfs = data.pdfs_previews;
        this.total_number_of_pages = data.count
      });
      return;
    }

    this.page = 1;

    let searchDto: PdfSearch = {
      title: this.search_title,
      author: this.search_author,
      tags: this.search_tags,
      page: this.page,
      size: this.size
    }

    this.pdfService.search(searchDto).subscribe({
      next: data => {
        this.pdfs = data.pdfs_previews;
        this.total_number_of_pages = data.count;
        //if page == 1 and data.pdf_previews.length == 0
        //  show error msg that there are no search results
      },
      error: err => {/*TODO: Error handling*/}
    })

  }



}
