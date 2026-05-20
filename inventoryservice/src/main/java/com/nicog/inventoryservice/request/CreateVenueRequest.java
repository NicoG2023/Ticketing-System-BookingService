package com.nicog.inventoryservice.request;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
public class CreateVenueRequest {

    private String name;
    private String address;
    private Long totalCapacity;
}
