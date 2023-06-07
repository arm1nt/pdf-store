package com.arm1nt.pdfstore.endpoint.dto;

import com.arm1nt.pdfstore.endpoint.validation.NullOrNotBlank;
import lombok.Data;
import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
public class PdfMetaDataDto {

    @NullOrNotBlank
    private String title;

    @NullOrNotBlank
    private String author;

    @NullOrNotBlank
    private String comments;

}
