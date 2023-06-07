package com.arm1nt.pdfstore.repository.impl;

import com.arm1nt.pdfstore.endpoint.dto.PdfMetaDataDto;
import com.arm1nt.pdfstore.entity.Pdf;
import com.arm1nt.pdfstore.repository.PdfRepository;
import com.arm1nt.pdfstore.repository.StorageDao;

import org.apache.pdfbox.pdmodel.PDDocument;
import org.apache.pdfbox.pdmodel.PDDocumentInformation;
import org.apache.pdfbox.rendering.PDFRenderer;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Repository;
import org.springframework.web.multipart.MultipartFile;

import javax.imageio.ImageIO;
import java.awt.image.BufferedImage;
import java.io.BufferedInputStream;
import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.IOException;
import java.lang.invoke.MethodHandles;
import java.util.ArrayList;
import java.util.List;
import java.util.UUID;
import java.util.concurrent.Callable;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

@Repository
public class StorageDaoImpl implements StorageDao {

    private static final Logger LOGGER = LoggerFactory.getLogger(MethodHandles.lookup().lookupClass());

    private final PdfRepository pdfRepository;

    @Autowired
    public StorageDaoImpl(PdfRepository pdfRepository) {
        this.pdfRepository = pdfRepository;
    }

    /**
     * Store the relevant metdata of the pdf document.
     *
     * @param file
     * @param document
     * @param picture
     * @return
     */
    private Pdf storePdfMetaData(MultipartFile file, PDDocument document, String picture) {
        LOGGER.trace("storePdfMetaData({}, {})", file, document);

        PDDocumentInformation info = document.getDocumentInformation();
        Pdf pdf = Pdf.builder()
                .fileName(file.getOriginalFilename())
                .title(info.getTitle())
                .author(info.getAuthor())
                .pages(document.getNumberOfPages())
                .picture(picture)
                .build();

        return this.pdfRepository.save(pdf);
    }

    /**
     * Create thumbnail of the first page of the document and return a base64 encoded jpeg picture.
     *
     * @param document pdf document
     * @return base64 encoded picture
     */
    private String createPdfThumbnail(PDDocument document) {
        LOGGER.trace("createPdfThumbnail({})", document);

        String base64Image = null;

        try {
            PDFRenderer pdfRenderer = new PDFRenderer(document);
            BufferedImage image = pdfRenderer.renderImage(0, 0.25f);

            ByteArrayOutputStream outputStream = new ByteArrayOutputStream();
            ImageIO.write(image, "jpeg", outputStream);
            byte[] imageBytes = outputStream.toByteArray();
            base64Image = java.util.Base64.getEncoder().encodeToString(imageBytes);

        } catch (IOException e) {
            //log exception
            e.printStackTrace();

        }
        return base64Image;
    }

    /**
     * Store pdf on file system
     *
     * @param file
     * @param pdfId
     * @throws IOException
     */
    private void storePdfOnFileSystem(MultipartFile file, UUID pdfId) throws IOException {
        LOGGER.trace("storePdfOnFileSystem({}, {})", file, pdfId);

        String filePath;
        if (pdfId == null) {
            filePath = System.getenv("PDFSTORE_FILEPATH") + System.getProperty("file.separator") + file.getOriginalFilename();
        } else {
            filePath = System.getenv("PDFSTORE_FILEPATH") + System.getProperty("file.separator") + pdfId.toString() + ".pdf";
        }

        File outputFile = new File(filePath);
        file.transferTo(outputFile);
    }

    @Override
    public void savePdfs(MultipartFile[] files, PdfMetaDataDto pdfMetaDataDto) {
        LOGGER.trace("savePdfs()");

        ExecutorService executor = Executors.newFixedThreadPool(Runtime.getRuntime().availableProcessors());

        List<Callable<Void>> callables = new ArrayList<>();
        for(MultipartFile file : files) {
            Callable<Void> callable = () -> {
                try {
                    UUID pdfId = null;
                    try(BufferedInputStream inputStream = new BufferedInputStream(file.getInputStream())) {
                        PDDocument document = PDDocument.load(inputStream);

                        String base64Image = this.createPdfThumbnail(document);
                        pdfId = this.storePdfMetaData(file, document, base64Image).getId();

                        document.close();
                    } catch (Exception e) {
                        e.printStackTrace();
                    }

                    this.storePdfOnFileSystem(file, pdfId);

                } catch (Exception e) {
                    e.printStackTrace();
                    System.out.println(e.getMessage());
                }
                return null;
            };
            callables.add(callable);
        }

        try {
            executor.invokeAll(callables);
        } catch (InterruptedException e) {
            e.printStackTrace();
            System.out.println(e.getMessage());
        }
        executor.shutdown();
    }
}
