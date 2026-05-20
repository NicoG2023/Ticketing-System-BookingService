package com.nicog.inventoryservice.request;

import java.math.BigDecimal;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
public class CreateEventRequest {

    private String name;
    private Long totalCapacity;
    private Long venueId;
    private BigDecimal ticketPrice;
}
