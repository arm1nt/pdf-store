export interface PdfOverviewInfo {
  pdfs_previews:  PdfOverviewDetails[],
  count: number
}

export interface PdfOverviewDetails {
  id: string,
  title: string,
  picture: string,
}
