package com.arm1nt.pdfstore.endpoint.dto;

import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
public class UserLoginDto {

    private String username;

    private String password;
}
