package com.arm1nt.pdfstore.service;

import com.arm1nt.pdfstore.endpoint.dto.PdfMetaDataDto;
import com.arm1nt.pdfstore.entity.Pdf;
import org.springframework.web.multipart.MultipartFile;

public interface PdfService {

    void storePdfs(MultipartFile[] files, PdfMetaDataDto pdfMetaDataDto);
}
