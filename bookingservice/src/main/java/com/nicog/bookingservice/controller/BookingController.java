package com.nicog.bookingservice.controller;

import com.nicog.bookingservice.request.BookingRequest;
import com.nicog.bookingservice.response.BookingResponse;
import com.nicog.bookingservice.service.BookingService;
import org.springframework.security.core.annotation.AuthenticationPrincipal;
import org.springframework.security.oauth2.jwt.Jwt;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/v1")
public class BookingController {

    private final BookingService bookingService;

    public BookingController(final BookingService bookingService) {
        this.bookingService = bookingService;
    }

    @PostMapping(
        consumes = "application/json",
        produces = "application/json",
        path = "/booking"
    )
    public BookingResponse createBooking(
        @AuthenticationPrincipal Jwt jwt,
        @RequestBody BookingRequest request
    ) {
        String userId = jwt.getSubject();
        String email = jwt.getClaimAsString("email");
        String username = jwt.getClaimAsString("preferred_username");

        return bookingService.createBooking(request, userId, email, username);
    }
}
