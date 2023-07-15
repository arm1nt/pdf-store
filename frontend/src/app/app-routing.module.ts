import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import {UploadComponent} from "./components/upload/upload.component";
import {OverviewComponent} from "./components/overview/overview.component";
import {PdfDetailViewComponent} from "./components/pdf-detail-view/pdf-detail-view.component";

const routes: Routes = [
  {path: '', redirectTo: '/overview', pathMatch: 'full'},
  {path: 'upload', component: UploadComponent},
  {path: 'overview', component: OverviewComponent},
  {path: 'detail/:id', component: PdfDetailViewComponent},
  {path: '*', component: OverviewComponent},
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
