package com.arm1nt.pdfstore.repository;

import com.arm1nt.pdfstore.endpoint.dto.PdfMetaDataDto;
import org.springframework.web.multipart.MultipartFile;

public interface StorageDao {

    void savePdfs(MultipartFile files[], PdfMetaDataDto pdfMetaDataDto);
}
