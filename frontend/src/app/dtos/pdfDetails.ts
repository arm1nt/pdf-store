export interface PdfDetails {
  id: string,
  title: string,
  author: string,
  pages: number,
  comments: string,
  tags: string[],
  last_accessed: Date,
  uploaded: Date,
  picture: string,
  pdf: Uint8Array,
}
