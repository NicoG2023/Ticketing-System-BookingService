package com.nicog.bookingservice.request;

import lombok.Data;

@Data
public class BookingRequest {

    private Long eventId;
    private Long ticketCount;
}
