package com.nicog.inventoryservice.simulation;

import lombok.Builder;
import lombok.Data;

@Data
@Builder
public class DirtyReadSimulationState {

    private Long eventId;
    private String eventName;

    private Long initialConfirmedCapacity;

    private Long sessionAUncommittedCapacity;
    private Long sessionBReadCapacity;
    private Long finalConfirmedCapacity;

    private Boolean sessionAStarted;
    private Boolean sessionAHasUncommittedWrite;
    private Boolean sessionBHasRead;
    private Boolean rollbackExecuted;
    private Boolean dirtyReadDetected;

    private String currentStep;

    private String sessionAStatus;
    private String sessionBStatus;
    private String diagnosis;
}
