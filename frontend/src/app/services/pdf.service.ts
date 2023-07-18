import { Injectable } from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Observable} from "rxjs";
import {Globals} from "../global/globals";
import {PdfOverviewInfo} from "../dtos/pdfOverview";
import {PdfDetails} from "../dtos/pdfDetails";
import {PdfDownload} from "../dtos/pdfDownload";

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

    return this.httpClient.get<PdfOverviewInfo>(`${this.pdfBaseUri}pdfs?page=${page}&size=${size}`);
  }

  getMetadataById(id: string): Observable<PdfDetails> {
    return this.httpClient.get<PdfDetails>(`${this.pdfBaseUri}pdfs/metadata/${id}`);
  }

  //TODO: change return type of observable
  getById(id: string): Observable<PdfDownload> {
    return this.httpClient.get<PdfDownload>(`${this.pdfBaseUri}pdfs/${id}`);
  }

}
