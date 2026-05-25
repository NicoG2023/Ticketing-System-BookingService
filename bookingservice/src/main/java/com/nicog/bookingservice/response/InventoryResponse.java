package com.nicog.bookingservice.response;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

@Data
@NoArgsConstructor
@AllArgsConstructor
@Builder
public class InventoryResponse {

    private Long eventId;
    private String event;
    private Long capacity;
    private String venue;
    private Double ticketPrice;
}
