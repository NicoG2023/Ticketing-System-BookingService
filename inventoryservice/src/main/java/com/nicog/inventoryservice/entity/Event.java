package com.nicog.inventoryservice.entity;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

@Getter
@Setter
@Builder
@AllArgsConstructor
@NoArgsConstructor
public class Event {

    private Long id;
    private String name;
    private Long totalCapacity;
    private Long leftCapacity;
    private Long venueId;
    private Double ticketPrice;
}
