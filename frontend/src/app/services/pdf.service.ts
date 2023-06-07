import { Injectable } from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Observable} from "rxjs";
import {Globals} from "../global/globals";
import {UploadPdfsDto} from "../dtos/pdfUpload";

@Injectable({
  providedIn: 'root'
})
export class PdfService {

  private pdfBaseUri: string = this.globals.backendUri + '/pdfs';

  constructor(
    private httpClient: HttpClient,
    private globals: Globals
  ) { }

  uploadFile(files: File[]): Observable<boolean> {

    const formData = new FormData();
    for (let i = 0; i < files.length; ++i) {
      formData.append(`files`, files[i], files[i].name);
    }

    return this.httpClient.post<boolean>(this.pdfBaseUri, formData);
  }
}
