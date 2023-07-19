import { Injectable } from '@angular/core';
import {HttpClient, HttpParams} from "@angular/common/http";
import {Observable} from "rxjs";
import {Globals} from "../global/globals";
import {PdfOverviewInfo} from "../dtos/pdfOverview";
import {PdfDetails} from "../dtos/pdfDetails";
import {PdfDownload} from "../dtos/pdfDownload";
import {PdfSearch} from "../dtos/pdfSearch";

@Injectable({
  providedIn: 'root'
})
export class PdfService {

  private pdfBaseUri: string = this.globals.backendUri + '/';

  constructor(
    private httpClient: HttpClient,
    private globals: Globals
  ) { }

  uploadFile(files: File[]): Observable<boolean> {

    const formData = new FormData();
    for (let i = 0; i < files.length; ++i) {
      formData.append(`file`, files[i], files[i].name);
    }

    return this.httpClient.post<boolean>(this.pdfBaseUri, formData);
  }

  getAllFilesPaged(page: number, size: number): Observable<PdfOverviewInfo> {

    let params = new HttpParams();
    params = params.set('page', page);
    params = params.set('size', size);

    return this.httpClient.get<PdfOverviewInfo>(`${this.pdfBaseUri}pdfs`, {params});
  }

  getMetadataById(id: string): Observable<PdfDetails> {
    return this.httpClient.get<PdfDetails>(`${this.pdfBaseUri}pdfs/metadata/${id}`);
  }

  getById(id: string): Observable<PdfDownload> {
    return this.httpClient.get<PdfDownload>(`${this.pdfBaseUri}pdfs/${id}`);
  }

  search(search: PdfSearch): Observable<PdfOverviewInfo> {

    let params = new HttpParams();

    if (search.title) {
      params = params.set('title', search.title);
    }

    if (search.author) {
      params = params.set('author', search.author);
    }

    if (search.tags) {
      params = params.set('tag', search.tags);
    }

    params = params.set('page', search.page);
    params = params.set('size', search.size);


    return this.httpClient.get<PdfOverviewInfo>(`${this.pdfBaseUri}pdfs/search`, {params});

  }

}
