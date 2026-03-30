package com.nicog.bookingservice.request;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

@Data
@AllArgsConstructor
@Builder
@NoArgsConstructor
public class BookingRequest {
    private Long userId;
    private Long eventId;
    private Long ticketCount;
}
