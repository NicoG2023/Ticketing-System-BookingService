package com.nicog.inventoryservice.response;

import lombok.Builder;
import lombok.Data;

@Data
@Builder
public class DirtyReadSimulationResponse {

    private Long eventId;
    private String eventName;

    private String currentStep;

    private Long initialConfirmedCapacity;
    private Long sessionAUncommittedCapacity;
    private Long sessionBReadCapacity;
    private Long finalConfirmedCapacity;

    private Boolean sessionAStarted;
    private Boolean sessionAHasUncommittedWrite;
    private Boolean sessionBHasRead;
    private Boolean rollbackExecuted;
    private Boolean dirtyReadDetected;

    private String sessionAStatus;
    private String sessionBStatus;

    private String diagnosis;
    private String explanation;
    private String control;
}
