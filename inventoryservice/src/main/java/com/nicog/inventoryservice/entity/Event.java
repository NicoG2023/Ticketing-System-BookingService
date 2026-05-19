package com.nicog.inventoryservice.entity;

import java.math.BigDecimal;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

@Getter
@Setter
@AllArgsConstructor
@NoArgsConstructor
public class Event {

    private Long id;
    private String name;
    private Long totalCapacity;
    private Long leftCapacity;
    private Long venueId;
    private BigDecimal ticketPrice;
}
