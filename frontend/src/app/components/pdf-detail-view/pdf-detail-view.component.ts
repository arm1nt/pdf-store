import {Component, OnInit} from '@angular/core';
import {ActivatedRoute, Router} from "@angular/router";
import {PdfService} from "../../services/pdf.service";
import {PdfDetails} from "../../dtos/pdfDetails";
import {MatSnackBar, MatSnackBarConfig} from "@angular/material/snack-bar";
import {COMMA, ENTER} from '@angular/cdk/keycodes';
import {UpdatePdf} from "../../dtos/updatePdf";
import {MatDialog} from "@angular/material/dialog";
import {DeleteDialogComponent} from "../delete-dialog/delete-dialog.component";
import {MatChipInputEvent} from "@angular/material/chips";

@Component({
  selector: 'app-pdf-detail-view',
  templateUrl: './pdf-detail-view.component.html',
  styleUrls: ['./pdf-detail-view.component.css']
})
export class PdfDetailViewComponent implements OnInit {

  editMode: boolean = false;

  pdfUpdate: UpdatePdf = {
    id: '',
    title: '',
    author: '',
    comments: '',
    picture: '',
    tags: []
  };

  pdfId: string | null | undefined;
  pdfDetails: PdfDetails | undefined;

  addOnBlur = true;
  readonly separatorKeysCodes = [ENTER, COMMA] as const;

  private snackbarConfig: MatSnackBarConfig = {
    duration: 2000,
  };

  constructor(
    private pdfService: PdfService,
    private router: Router,
    private route: ActivatedRoute,
    private _snackBar: MatSnackBar,
    private dialog: MatDialog
  ) {
  }

  ngOnInit() {

    let id = this.route.snapshot.paramMap.get('id');
    this.pdfId = id;

    if (id === null) {
      this.router.navigate(["/overview"]);
      return;
    }

    this.pdfService.getMetadataById(id.toString()).subscribe({
      next: value => {
        this.pdfDetails = value;
        this.pdfUpdate = {
          id: value.id,
          title: value.title,
          author: value.author,
          comments: value.comments,
          tags: JSON.parse(JSON.stringify(value.tags)),
          picture: value.picture
        }
      },
      error: err => {
        if (err.status === 404) {
          this._snackBar.open("No pdf with given id found", "Close", this.snackbarConfig);
        } else if (err.status === 400) {
          this._snackBar.open("Invalid ID given", "Close", this.snackbarConfig);
        } else {
          this._snackBar.open("There was an error retrieving the requested pdf", "Close", this.snackbarConfig);
        }
        this.router.navigate(["/overview"]);
      }
      });
  }

  toggleEditMode() {
    if (this.editMode) {
      this.revertEditChanges();
    }

    this.editMode = !this.editMode;
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
      error: err => {
        this._snackBar.open("There was an error loading the pdf", "Close", this.snackbarConfig);
      }
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

  deletePdf() {
    const dialogRef = this.dialog.open(DeleteDialogComponent);

    dialogRef.afterClosed().subscribe(result => {
      if (result) {
        if (!this.pdfId) {
          return;
        }

        this.pdfService.deletePdf(this.pdfId).subscribe({
          next: _ => {
            this._snackBar.open("Successfully deleted pdf", "Close", this.snackbarConfig);
            this.router.navigate(["/overview"]);
          },
          error: err => {
            this._snackBar.open("There was an error deleting the pdf", "Close", this.snackbarConfig);
          }
        });
      }
    });
  }

  add(event: MatChipInputEvent) {
    const value = (event.value || '').trim();

    if (value) {

      if (this.pdfUpdate.tags.includes(value)) {
        this._snackBar.open("Tag already exists.", "Close", this.snackbarConfig);
        event.chipInput!.clear();
        return;
      }

      this.pdfUpdate.tags.push(value);
    }
    event.chipInput!.clear();
  }

  remove(tag: string) {
    const index = this.pdfUpdate.tags.indexOf(tag);

    if (index >= 0) {
      this.pdfUpdate.tags.splice(index, 1);
    }
  }

  revertEditChanges() {
    this.pdfUpdate.author = this.pdfDetails?.author || "";
    this.pdfUpdate.title = this.pdfDetails?.title || "";
    if (this.pdfDetails) {
      this.pdfUpdate.tags = JSON.parse(JSON.stringify(this.pdfDetails.tags));
    } else {
      this.pdfUpdate.tags = [];
    }
    this.pdfUpdate.comments = this.pdfDetails?.comments || "";
  }

  cancelEdit() {
    this.revertEditChanges();
    this.editMode = false;
  }

  saveEdit() {
    this.pdfService.updatePdf(this.pdfUpdate).subscribe({
      next: _ => {
        this.editMode = false;
        this._snackBar.open("Successfully updated pdf", "Close", this.snackbarConfig);
      },
      error: err => {
        this._snackBar.open("There was an error updating the pdf.", "Close", this.snackbarConfig);
      }
    });
  }


}
