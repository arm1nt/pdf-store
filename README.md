# PdfStore

WebApp with an Angular frontend, an Actix-Web Backend and a Postgres database.
<br/>
<br/>
## Features

- Upload Pdfs

Upload one single pdf or multiple pdfs at once by using the select button or via drag and drop.

- Overview

Paginated overview of all uploaded pdfs, with an automatically generated preview image of the pdf cover.

- Search Pdfs

Search pdfs based on their title, author or an associated tag.

- Detail View

Detail view of the pdf which displays additional information like author, upload date, number of pages, associated tags, comments.

In this view the pdf can also be downloaded or opened.

- Edit Pdfs

Selected attributes of the pdf can be changed in this view.

- Delete Pdfs

Delete stored pdfs with an additional dialog to confirm the delete intent.
<br/>
<br/>
## Requirements

Requires the Pdfium DLL.

The pdfium library can be downloaded from: https://github.com/bblanchon/pdfium-binaries/releases
