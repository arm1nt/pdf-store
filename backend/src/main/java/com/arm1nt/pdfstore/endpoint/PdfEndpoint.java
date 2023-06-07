package com.arm1nt.pdfstore.endpoint;

import com.arm1nt.pdfstore.endpoint.dto.PdfMetaDataDto;
import com.arm1nt.pdfstore.service.PdfService;
import jakarta.validation.Valid;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.web.bind.annotation.CrossOrigin;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.ResponseStatus;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.multipart.MultipartFile;

import java.lang.invoke.MethodHandles;

@CrossOrigin
@RestController
@RequestMapping("/api/v1/pdfs")
public class PdfEndpoint {

    private static final Logger LOGGER = LoggerFactory.getLogger(MethodHandles.lookup().lookupClass());

    private final PdfService pdfService;

    @Autowired
    public PdfEndpoint(PdfService pdfService) {
        this.pdfService = pdfService;
    }

    @PostMapping
    @ResponseStatus(code = HttpStatus.CREATED)
    public void uploadPdfs(@RequestParam(value = "files", required = true) MultipartFile[] files,
                              @Valid @RequestParam(value = "metaData", required = false) PdfMetaDataDto pdfMetaDataDto) {

        LOGGER.info("POST /api/v1/pdfs");

        this.pdfService.storePdfs(files, pdfMetaDataDto);
    }

}
