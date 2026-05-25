package com.nicog.inventoryservice.response;

import lombok.Builder;
import lombok.Data;

@Data
@Builder
public class EventInventoryResponse {

    private Long eventId;
    private String event;
    private Long capacity;
    private String venue;
    private Double ticketPrice;
}
