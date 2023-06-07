package com.arm1nt.pdfstore.repository;

import com.arm1nt.pdfstore.entity.Pdf;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

import java.util.UUID;

@Repository
public interface PdfRepository extends JpaRepository<Pdf, UUID> {
}
