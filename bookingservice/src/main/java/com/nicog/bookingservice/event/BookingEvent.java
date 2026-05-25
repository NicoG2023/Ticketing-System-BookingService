package com.nicog.bookingservice.event;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

@Data
@NoArgsConstructor
@AllArgsConstructor
@Builder
public class BookingEvent {

    private String userId;
    private Long eventId;
    private Long ticketCount;
    private Double totalPrice;
}
