package com.arm1nt.pdfstore.service.impl;

import com.arm1nt.pdfstore.entity.User;
import com.arm1nt.pdfstore.repository.UserRepository;
import com.arm1nt.pdfstore.service.UserService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

@Service
@Transactional(readOnly = true)
public class UserServiceImpl implements UserService {

    private final UserRepository userRepository;

    @Autowired
    public UserServiceImpl(UserRepository userRepository) {
        this.userRepository = userRepository;
    }

    @Transactional
    public User register() {
        User user = User.builder()
                .email("email")
                .username("username")
                .password("password")
                .build();
        User savedUser = this.userRepository.save(user);

        return savedUser;
    }
}
