package com.nicog.inventoryservice.simulation;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
public class LostUpdateSimulationState {

    private Long eventId;
    private Long initialCapacity;

    private Long requestAReadCapacity;
    private Long requestACalculatedCapacity;
    private Boolean requestACommitted;

    private Long requestBReadCapacity;
    private Long requestBCalculatedCapacity;
    private Boolean requestBCommitted;

    private String requestAStatus;
    private String requestBStatus;
}
