package com.nicog.inventoryservice.entity;

import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

@Getter
@Setter
@AllArgsConstructor
@NoArgsConstructor
public class Venue {

    private Long id;
    private String name;
    private String address;
    private Long totalCapacity;
}
