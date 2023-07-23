import {Component, OnInit, ViewChild} from '@angular/core';
import {PdfOverviewDetails, PdfOverviewInfo} from "../../dtos/pdfOverview";
import {PdfService} from "../../services/pdf.service";
import {MatPaginator, PageEvent} from "@angular/material/paginator";
import {PdfSearch} from "../../dtos/pdfSearch";
import {MatSnackBar, MatSnackBarConfig} from "@angular/material/snack-bar";
import {ActivatedRoute, Router} from "@angular/router";
import {take} from "rxjs";

@Component({
  selector: 'app-overview',
  templateUrl: './overview.component.html',
  styleUrls: ['./overview.component.css']
})
export class OverviewComponent implements OnInit {
  searchMode = false;

  loadError = false;
  noResults = false;

  pdfs: PdfOverviewDetails[] = [];
  page: number = 1;
  size: number = 24;
  total_number_of_pages: number | undefined = undefined;
  expand_detail_view = false;

  //Bound to the value of the input field
  search_bind_title:  string | undefined = undefined;
  search_bind_author: string | undefined = undefined;
  search_bind_tags: string | undefined = undefined;

  //Store the search values for each search
  search_title:  string | undefined = undefined;
  search_author: string | undefined = undefined;
  search_tags: string | undefined = undefined;


  private snackBarConfig: MatSnackBarConfig = {
    duration: 2000
  };

  constructor(
    private pdfService: PdfService,
    private _snackBar: MatSnackBar,
    private activatedRoute: ActivatedRoute,
    private router: Router,
  ) {
  }

  ngOnInit() {

    this.activatedRoute.queryParams.pipe(take(1)).subscribe({
      next: params => {
          this.search_title =  params['title'] || undefined;
          this.search_author = params['author'] || undefined;
          this.search_tags =  params['tag'] || undefined;
          this.page = params['page'] || 1;

          this.search_bind_title = this.search_title;
          this.search_bind_author = this.search_author;
          this.search_bind_tags = this.search_tags;

          if (this.search_title || this.search_author || this.search_tags) {
            this.searchMode = true;

            if (this.search_author || this.search_tags) {
              this.expand_detail_view = true;
            }

            this.searchPdfs(undefined, undefined);
            return;
          }

          this.adjustQueryParamsUrl(null, null, null, this.page);
          this.getAllPdfs(undefined, undefined);
        },
      error: err => {
        this.adjustQueryParamsUrl(null, null, null, 1);
        this.getAllPdfs(undefined, undefined);
      }
      });
  }

  getNextPage(e: PageEvent) {
    this.page = e.pageIndex+1;

    if (this.searchMode) {
      this.adjustQueryParamsUrl(this.search_title, this.search_author, this.search_tags, e.pageIndex+1);
      this.searchPdfs(e.pageIndex+1, e.pageSize);

      return;
    }

    this.adjustQueryParamsUrl(null, null, null, e.pageIndex+1);
    this.getAllPdfs(e.pageIndex+1, e.pageSize);
  }

  showDetailView() {
    this.expand_detail_view = !this.expand_detail_view;
  }

  clear() {
    this.search_bind_title = undefined;
    this.search_bind_author = undefined;
    this.search_bind_tags = undefined;
  }

  search() {
    this.searchMode = true;

    if (this.search_bind_title?.trim().length == 0) {
      this.search_bind_title = undefined;
    }

    if (this.search_bind_author?.trim().length == 0) {
      this.search_bind_author = undefined;
    }

    if (this.search_bind_tags?.trim().length == 0) {
      this.search_bind_tags = undefined;
    }

    if ( this.search_bind_tags == undefined &&
      this.search_bind_title == undefined &&
      this.search_bind_author == undefined) {
      this.page = 1;
      this.searchMode = false;

      this.adjustQueryParamsUrl(null, null, null, this.page);
      this.getAllPdfs(undefined, undefined);

      return;
    }

    this.search_title = this.search_bind_title;
    this.search_author = this.search_bind_author;
    this.search_tags = this.search_bind_tags;
    this.page = 1;

    this.adjustQueryParamsUrl(this.search_title, this.search_author, this.search_tags, 1);
    this.searchPdfs(undefined, undefined);
  }

  reloadPdfs() {
    this.loadError = false;

    if (this.search_title || this.search_author || this.search_tags) {
      this.searchPdfs(undefined, undefined);
    } else {
      this.getAllPdfs(undefined, undefined);
    }
  }


  //Precond: either both arguments are undefined or both are of type number
  private searchPdfs(pageIndex: number | undefined, pageSize: number | undefined) {

    if (!pageIndex || !pageSize) {
      pageIndex = this.page;
      pageSize = this.size;
    }

    let searchDto: PdfSearch = {
      title: this.search_title,
      author: this.search_author,
      tags: this.search_tags,
      page: pageIndex,
      size: pageSize
    };

    this.pdfService.search(searchDto).subscribe({
      next: data => {
        this.pdfs = data.pdfs_previews;
        this.total_number_of_pages = data.count;

        this.noResults = data.count == 0 || data.pdfs_previews.length == 0;
      },
      error: err => {
        this._snackBar.open("Error retrieving search results", "Close", this.snackBarConfig);
        this.loadError = true;
      }
    });
  }

  //Precond: either both arguments are undefined or both are of type number
  private getAllPdfs(pageIndex: number | undefined, pageSize: number | undefined) {

    if (!pageIndex || ! pageSize) {
      pageIndex = this.page;
      pageSize = this.size;
    }

    this.pdfService.getAllFilesPaged(pageIndex, pageSize).subscribe({
      next: data => {
        this.pdfs = data.pdfs_previews;
        this.total_number_of_pages = data.count;

        this.noResults = data.count == 0 || data.pdfs_previews.length == 0;

      },
      error: err => {
        this._snackBar.open("Error retrieving pdfs", "Close", this.snackBarConfig);
        this.loadError = true;
      }
    });
  }

  private adjustQueryParamsUrl( title: string | null | undefined, author: string | null | undefined,
    tag: string | null | undefined, page: number | null | undefined
  ) {
    this.router.navigate([], {
      relativeTo: this.activatedRoute,
      queryParams: {
        'tag': tag,
        'author': author,
        'title': title,
        'page': page,
      },
      queryParamsHandling: "merge"
    });
  }



}
