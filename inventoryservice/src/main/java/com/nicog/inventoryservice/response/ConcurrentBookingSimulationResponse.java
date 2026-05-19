package com.nicog.inventoryservice.response;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
public class ConcurrentBookingSimulationResponse {

    private Long eventId;
    private Long initialCapacity;
    private Long finalCapacity;

    private String requestAStatus;
    private String requestBStatus;

    private String conclusion;
}
