package com.nicog.inventoryservice.response;

import lombok.Builder;
import lombok.Data;

@Data
@Builder
public class LostUpdateSimulationResponse {

    private Long eventId;
    private Long initialCapacity;
    private Long requestACalculatedCapacity;
    private Long requestBCalculatedCapacity;
    private Long finalCapacity;
    private Long expectedCapacity;
    private Boolean lostUpdateOccurred;
    private String requestAStatus;
    private String requestBStatus;
    private String explanation;
    private String control;
}
