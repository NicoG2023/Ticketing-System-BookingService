package com.nicog.orderservice.entity;

import java.time.LocalDateTime;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

@Data
@NoArgsConstructor
@AllArgsConstructor
@Builder
public class Order {

    private String id;
    private Double totalPrice;
    private Long ticketCount;
    private LocalDateTime placedAt;
    private String customerId;
    private Long eventId;
    private String status;
}
