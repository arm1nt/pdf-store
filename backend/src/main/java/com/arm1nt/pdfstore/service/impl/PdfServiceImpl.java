package com.arm1nt.pdfstore.service.impl;

import com.arm1nt.pdfstore.endpoint.dto.PdfMetaDataDto;
import com.arm1nt.pdfstore.repository.PdfRepository;
import com.arm1nt.pdfstore.repository.StorageDao;
import com.arm1nt.pdfstore.service.PdfService;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import org.springframework.web.multipart.MultipartFile;

import java.lang.invoke.MethodHandles;


@Service
@Transactional(readOnly = true)
public class PdfServiceImpl implements PdfService {

    private static final Logger LOGGER = LoggerFactory.getLogger(MethodHandles.lookup().lookupClass());

    private final StorageDao storageDao;
    private final PdfRepository pdfRepository;

    @Autowired
    public PdfServiceImpl(StorageDao storageDao, PdfRepository pdfRepository) {
        this.storageDao = storageDao;
        this.pdfRepository = pdfRepository;
    }

    @Override
    public void storePdfs(MultipartFile[] files, PdfMetaDataDto pdfMetaDataDto) {
        LOGGER.trace("storePdfs()");

        this.storageDao.savePdfs(files, pdfMetaDataDto);
    }
}
